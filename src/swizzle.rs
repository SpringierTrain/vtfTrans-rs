
use std::collections::btree_map::Range;

fn generate_swizzle_masks(width:usize ,height:usize ,depth:usize ,
                        mut mask_x:usize ,mut mask_y:usize ,mut mask_z:usize){
    let mut x:usize = 0;
    let mut y:usize = 0;
    let mut z:usize = 0;

    let mut bit:usize = 1;
    let mut mask_bit:usize = 1;

    let mut done:bool;
    loop{
        done = true;
        if (bit < width) { x |= mask_bit; mask_bit <<= 1; done = false; }
        if (bit < height) { y |= mask_bit; mask_bit <<= 1; done = false; }
        if (bit < depth) { z |= mask_bit; mask_bit <<= 1; done = false; }
        bit <<= 1;
        if(done){break;}
    }
    assert!(((x ^ y) ^ z) == (mask_bit - 1));
    mask_x = x;
    mask_y = y;
    mask_z = z;

}
fn fill_pattern(pattern:usize,value:usize)-> usize{
    let mut value = value;
    let mut result:usize = 0;
    let mut bit:usize = 0;
    while(value>0){
        if ((pattern & bit)>0) {
            /* Copy bit to result */
            result |= if (value & 1)>0 {bit}else{0};
            value >>= 1;
        }
        bit <<= 1;
    }
    result
}

fn get_swizzled_offset(x:usize,y:usize,z:usize,
    mask_x:usize,mask_y:usize,mask_z:usize,
    bytes_per_pixel:usize) -> usize{
        bytes_per_pixel * (fill_pattern(mask_x, x)
                           | fill_pattern(mask_y, y)
                           | fill_pattern(mask_z, z))
}
fn unswizzle_box(src_buf:Vec<u8>,width:usize,height:usize,depth:usize,row_pitch:usize,slice_pitch:usize,bytes_per_pixel:usize) -> Vec<u8>{
    let mut mask_x =0;
    let mut mask_y=0;
    let mut mask_z=0;
    generate_swizzle_masks(width, height, depth, mask_x, mask_y, mask_z);
    
    let mut x:usize;
    let mut y:usize;
    let mut z:usize;
    let mut dst_buf:Vec<u8> = src_buf.clone();
    let mut dst_off = 0;
    for z in 0..depth {
        for y in 0..height{
            for x in 0..width{
                let src = get_swizzled_offset(x, y, z, mask_x, mask_y, mask_z,
                                          bytes_per_pixel) as usize;
                let dst = (dst_off + y*row_pitch+x*bytes_per_pixel) as usize;
                dst_buf[dst..dst+(bytes_per_pixel as usize)].copy_from_slice(&src_buf[src..src+(bytes_per_pixel as usize)])
            }
        }
        dst_off+=slice_pitch
    }
    dst_buf
}

fn swizzle_box(src_buf:Vec<u8>,width:usize,height:usize,depth:usize,row_pitch:usize,slice_pitch:usize,bytes_per_pixel:usize) -> Vec<u8>{
    let mut mask_x =0;
    let mut mask_y=0;
    let mut mask_z=0;
    generate_swizzle_masks(width, height, depth, mask_x, mask_y, mask_z);
    
    let mut x:usize;
    let mut y:usize;
    let mut z:usize;
    let mut dst_buf:Vec<u8> = src_buf.clone();
    let mut src_off = 0;
    for z in 0..depth {
        for y in 0..height{
            for x in 0..width{
                let src = (src_off + y*row_pitch+x*bytes_per_pixel) as usize;
                let dst = get_swizzled_offset(x, y, 0, mask_x, mask_y, 0,
                    bytes_per_pixel) as usize;
                dst_buf[dst..dst+(bytes_per_pixel as usize)].copy_from_slice(&src_buf[src..src+(bytes_per_pixel as usize)])
            }
        }
        src_off+=slice_pitch
    }
    dst_buf
}

pub fn unswizzle_rect(src_buf:Vec<u8>,width:usize,height:usize,pitch:usize,bytes_per_pixel:usize)->Vec<u8>{
    unswizzle_box(src_buf, width, height, 1, pitch, 0, bytes_per_pixel)
}
pub fn swizzle_rect(src_buf:Vec<u8>,width:usize,height:usize,pitch:usize,bytes_per_pixel:usize)->Vec<u8>{
    swizzle_box(src_buf, width, height, 1, pitch, 0, bytes_per_pixel)
}