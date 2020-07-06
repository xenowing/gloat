use crate::vec4::*;

#[derive(Copy, Clone)]
pub enum BlendSrcFactor {
    Zero,
    One,
    SrcColor,
    SrcAlpha,
}

#[derive(Copy, Clone)]
pub enum BlendDstFactor {
    Zero,
    One,
    SrcAlpha,
    OneMinusSrcAlpha,
}

pub const TILE_DIM: usize = 16;
pub const TILE_PIXELS: usize = TILE_DIM * TILE_DIM;

pub const MAX_TEXTURE_DIM: usize = 128;
pub const MAX_TEXTURE_PIXELS: usize = MAX_TEXTURE_DIM * MAX_TEXTURE_DIM;

pub const W_FRACT_BITS: u32 = 8;
pub const W_INVERSE_FRACT_BITS: u32 = 30;
pub const Z_FRACT_BITS: u32 = 30; // Must be greater than 16
pub const ST_FRACT_BITS: u32 = 24;

pub struct ColorThrust {
    pub color_buffer: [u32; TILE_PIXELS],
    pub depth_buffer: [u16; TILE_PIXELS],

    // TODO: Split into four buffers for simultaneous reads for filtering
    pub texture_buffer: [u32; MAX_TEXTURE_PIXELS],
    pub texture_width_shift: u32,
    pub texture_height_shift: u32,

    pub w0_min: i32,
    pub w0_dx: i32,
    pub w0_dy: i32,
    pub w1_min: i32,
    pub w1_dx: i32,
    pub w1_dy: i32,
    pub w2_min: i32,
    pub w2_dx: i32,
    pub w2_dy: i32,
    pub w_inverse_min: i32,
    pub w_inverse_dx: i32,
    pub w_inverse_dy: i32,
    pub z_min: i32,
    pub z_dx: i32,
    pub z_dy: i32,
    pub s_min: i32,
    pub s_dx: i32,
    pub s_dy: i32,
    pub t_min: i32,
    pub t_dx: i32,
    pub t_dy: i32,

    pub depth_test_enable: bool,
    pub depth_mask_enable: bool,

    pub blend_src_factor: BlendSrcFactor,
    pub blend_dst_factor: BlendDstFactor,

    pub color: Vec4,
}

impl ColorThrust {
    pub fn new() -> ColorThrust {
        ColorThrust {
            color_buffer: [0; TILE_PIXELS],
            depth_buffer: [0; TILE_PIXELS],

            texture_buffer: [0; MAX_TEXTURE_PIXELS],
            texture_width_shift: 0,
            texture_height_shift: 0,

            w0_min: 0,
            w0_dx: 0,
            w0_dy: 0,
            w1_min: 0,
            w1_dx: 0,
            w1_dy: 0,
            w2_min: 0,
            w2_dx: 0,
            w2_dy: 0,
            w_inverse_min: 0,
            w_inverse_dx: 0,
            w_inverse_dy: 0,
            z_min: 0,
            z_dx: 0,
            z_dy: 0,
            s_min: 0,
            s_dx: 0,
            s_dy: 0,
            t_min: 0,
            t_dx: 0,
            t_dy: 0,

            depth_test_enable: false,
            depth_mask_enable: true,

            blend_src_factor: BlendSrcFactor::One,
            blend_dst_factor: BlendDstFactor::Zero,

            color: Vec4::zero(),
        }
    }

    pub fn rasterize_primitive(&mut self) {
        let mut w0_row = self.w0_min;
        let mut w1_row = self.w1_min;
        let mut w2_row = self.w2_min;
        let mut w_inverse_row = self.w_inverse_min;
        let mut z_row = self.z_min;
        let mut s_row = self.s_min;
        let mut t_row = self.t_min;

        // TODO: Clip to viewport bounds within tile
        for y in 0..TILE_DIM {
            let mut w0 = w0_row;
            let mut w1 = w1_row;
            let mut w2 = w2_row;
            let mut w_inverse = w_inverse_row;
            let mut z = z_row;
            let mut s = s_row;
            let mut t = t_row;

            for x in 0..TILE_DIM {
                if (w0 | w1 | w2) >= 0 {
                    let z = (z >> (Z_FRACT_BITS - 16)) as u16;
                    let buffer_index = y as usize * TILE_DIM + x as usize;
                    let depth_test_result = !self.depth_test_enable || z < self.depth_buffer[buffer_index];
                    const RESTORED_W_FRACT_BITS: u32 = 8; // Must be less than W_INVERSE_FRACT_BITS and ST_FRACT_BITS

                    fn inverse_approx(x: u32) -> u32 {
                        let shl = x.leading_zeros() & 31;
                        let normalized_x = x << shl;
                        // TODO: Why is 3 the magic number here? Is that dependent on the other constants? Can we determine shr a better way?
                        let shr = (64 - 2 * (W_INVERSE_FRACT_BITS - RESTORED_W_FRACT_BITS - 3) - shl) & 31;

                        let mut e = !normalized_x; // 2's complement approximation
                        let mut q = e;
                        for _ in 0..4 { // TODO: Is this the correct number of steps?
                            q += ((((q as u64) * (e as u64)) >> 32) as u32);
                            e = (((e as u64) * (e as u64)) >> 32) as u32;
                        }

                        return (q >> shr) | (1 << (32 - shr));
                    }
                    let w_approx = inverse_approx(w_inverse as _) as i32;

                    /*if x == 0 && y == 0 {
                        /*let one = 1 << W_INVERSE_FRACT_BITS;
                        let w = (one << W_INVERSE_FRACT_BITS) / (w_inverse as i64);
                        let w = (w >> (W_INVERSE_FRACT_BITS - RESTORED_W_FRACT_BITS)) as i32;*/
                        println!("***** w_inverse: 0x{:08x}, w: 0x{:08x}, w_approx: 0x{:08x}, error: {}", w_inverse, w, w_approx, (w_approx as i32) - (w as i32));
                    }*/

                    let w = w_approx;

                    let s = ((s >> RESTORED_W_FRACT_BITS) * w) as u32;
                    let t = ((t >> RESTORED_W_FRACT_BITS) * w) as u32;
                    let s_floor = s >> ST_FRACT_BITS;
                    let t_floor = t >> ST_FRACT_BITS;
                    const ST_FILTER_BITS: u32 = 4; // Must be less than ST_FRACT_BITS
                    let s_fract = (s >> (ST_FRACT_BITS - ST_FILTER_BITS)) & ((1 << ST_FILTER_BITS) - 1);
                    let t_fract = (t >> (ST_FRACT_BITS - ST_FILTER_BITS)) & ((1 << ST_FILTER_BITS) - 1);
                    let one_minus_s_fract = (1 << ST_FILTER_BITS) - s_fract;
                    let one_minus_t_fract = (1 << ST_FILTER_BITS) - t_fract;
                    let texel_color0 = self.fetch_texel(s_floor + 0, t_floor + 0);
                    let texel_color1 = self.fetch_texel(s_floor + 1, t_floor + 0);
                    let texel_color2 = self.fetch_texel(s_floor + 0, t_floor + 1);
                    let texel_color3 = self.fetch_texel(s_floor + 1, t_floor + 1);
                    let a_red = (texel_color0.0 * one_minus_s_fract + texel_color1.0 * s_fract) >> ST_FILTER_BITS;
                    let a_green = (texel_color0.1 * one_minus_s_fract + texel_color1.1 * s_fract) >> ST_FILTER_BITS;
                    let a_blue = (texel_color0.2 * one_minus_s_fract + texel_color1.2 * s_fract) >> ST_FILTER_BITS;
                    let a_alpha = (texel_color0.3 * one_minus_s_fract + texel_color1.3 * s_fract) >> ST_FILTER_BITS;
                    let b_red = (texel_color2.0 * one_minus_s_fract + texel_color3.0 * s_fract) >> ST_FILTER_BITS;
                    let b_green = (texel_color2.1 * one_minus_s_fract + texel_color3.1 * s_fract) >> ST_FILTER_BITS;
                    let b_blue = (texel_color2.2 * one_minus_s_fract + texel_color3.2 * s_fract) >> ST_FILTER_BITS;
                    let b_alpha = (texel_color2.3 * one_minus_s_fract + texel_color3.3 * s_fract) >> ST_FILTER_BITS;
                    let texel_red = (a_red * one_minus_t_fract + b_red * t_fract) >> ST_FILTER_BITS;
                    let texel_green = (a_green * one_minus_t_fract + b_green * t_fract) >> ST_FILTER_BITS;
                    let texel_blue = (a_blue * one_minus_t_fract + b_blue * t_fract) >> ST_FILTER_BITS;
                    let texel_alpha = (a_alpha * one_minus_t_fract + b_alpha * t_fract) >> ST_FILTER_BITS;
                    // TODO: Use properly interpolated vertex color
                    let src_color = self.color * 255.0 * Vec4::new(texel_red as f32, texel_green as f32, texel_blue as f32, texel_alpha as f32) / 256.0;

                    let src_scale_factors = match self.blend_src_factor {
                        BlendSrcFactor::Zero => Vec4::zero(),
                        BlendSrcFactor::One => Vec4::splat(255.0),
                        BlendSrcFactor::SrcColor => src_color,
                        BlendSrcFactor::SrcAlpha => Vec4::splat(src_color.w()),
                    };

                    let dst_color = self.color_buffer[buffer_index];
                    let dst_red = (dst_color >> 16) & 0xff;
                    let dst_green = (dst_color >> 8) & 0xff;
                    let dst_blue = (dst_color >> 0) & 0xff;
                    let dst_alpha = (dst_color >> 24) & 0xff;
                    let dst_color = Vec4::new(dst_red as f32, dst_green as f32, dst_blue as f32, dst_alpha as f32);
                    let dst_scale_factors = match self.blend_dst_factor {
                        BlendDstFactor::Zero => Vec4::zero(),
                        BlendDstFactor::One => Vec4::splat(255.0),
                        BlendDstFactor::SrcAlpha => Vec4::splat(src_color.w()),
                        BlendDstFactor::OneMinusSrcAlpha => Vec4::splat(255.0 - src_color.w()),
                    };

                    let color = (src_color * src_scale_factors + dst_color * dst_scale_factors) / 256.0;

                    let color = color.min(Vec4::splat(255.0));
                    let color_red = color.x().floor() as u32;
                    let color_green = color.y().floor() as u32;
                    let color_blue = color.z().floor() as u32;
                    let color_alpha = color.w().floor() as u32;
                    if depth_test_result {
                        self.color_buffer[buffer_index] = (color_alpha << 24) | (color_red << 16) | (color_green << 8) | (color_blue << 0);
                    }

                    self.depth_buffer[buffer_index] = if depth_test_result && self.depth_mask_enable {
                        z
                    } else {
                        self.depth_buffer[buffer_index]
                    };
                }

                w0 += self.w0_dx;
                w1 += self.w1_dx;
                w2 += self.w2_dx;
                w_inverse += self.w_inverse_dx;
                z += self.z_dx;
                s += self.s_dx;
                t += self.t_dx;
            }

            w0_row += self.w0_dy;
            w1_row += self.w1_dy;
            w2_row += self.w2_dy;
            w_inverse_row += self.w_inverse_dy;
            z_row += self.z_dy;
            s_row += self.s_dy;
            t_row += self.t_dy;
        }
    }

    fn fetch_texel(&self, s: u32, t: u32) -> (u32, u32, u32, u32) {
        let texture_width = 2 << self.texture_width_shift;
        let texture_height = 2 << self.texture_height_shift;
        let s = s as usize & (texture_width - 1);
        let t = t as usize & (texture_height - 1);
        let texel = self.texture_buffer[t * texture_width + s];
        let texel_red = (texel >> 16) & 0xff;
        let texel_green = (texel >> 8) & 0xff;
        let texel_blue = (texel >> 0) & 0xff;
        let texel_alpha = (texel >> 24) & 0xff;
        (texel_red, texel_green, texel_blue, texel_alpha)
    }
}
