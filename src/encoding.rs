// https://qoiformat.org/

const QOI_HEADER_SIZE: usize = 14;
const QOI_PADDING: [u8; 8] = [0,0,0,0,0,0,0,1];
const QOI_MAGIC: u32 = u32::from_be_bytes(*b"qoif");

const QOI_OP_RUN: u8 =      0xc0;
const QOI_OP_INDEX: u8 =    0x00;
const QOI_OP_DIFF: u8 =     0x40;
const QOI_OP_LUMA: u8 =     0x80;
const QOI_OP_RGB: u8 =      0xfe;
const QOI_OP_RGBA: u8 =     0xc0;

pub struct QoiDesc {
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub colorspace: u8
}

#[derive(Clone, Copy, PartialEq)]
struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

fn qoi_write32(bytes: &mut Vec<u8>, p: &mut usize, value: u32) {
    bytes[*p] = ((0xff000000 & value) >> 24) as u8;
    bytes[*p+1] = ((0x00ff0000 & value) >> 16) as u8;
    bytes[*p+2] = ((0x0000ff00 & value) >> 8) as u8;
    bytes[*p+3] = (0x000000ff & value) as u8;
    *p+=4;
}
fn qoi_color_hash(p: &Pixel) -> usize {
    (p.r*3 + p.g*5 + p.b*7 + p.a*11) as usize
}

pub fn qoi_encode(data: Vec<u8>, desc: QoiDesc) {

    let max_size: i32 = ( 
        desc.width * desc.height
        * (desc.channels + 1)as u32
        + QOI_HEADER_SIZE as u32
        + std::mem::size_of_val(&QOI_PADDING) as u32
    ) as i32;
    
    let mut p = 0;
    // let mut bytes: Vec<u8> = Vec::with_capacity(max_size as usize);
    let mut bytes: Vec<u8> = vec![0; max_size as usize];
    let mut index: Vec<Pixel> = vec![Pixel{r:0,g:0,b:0,a:0}; 64];


    // QOI header
    qoi_write32(&mut bytes, &mut p,  QOI_MAGIC);
    qoi_write32(&mut bytes, &mut p, desc.width);
    qoi_write32(&mut bytes, &mut p, desc.height);
    bytes[p] = desc.channels;
    bytes[p+1] = desc.colorspace;
    p+=2;


    let mut run = 0;
    let mut prev_px = Pixel{r:0, g: 0, b: 0, a: 255};
    let mut px = prev_px;

    let px_len: i32 = (desc.width * desc.height * desc.channels as u32) as i32;
    let px_end: i32 = px_len - desc.channels as i32;

    for px_pos in (0..px_len).step_by(desc.channels as usize) {
        // println!("{px_pos}");
        let idx = px_pos as usize;
        px.r = data[idx];
        px.g = data[idx + 1];
        px.b = data[idx + 2];
        
        if px == prev_px {
            run+=1;
            if run == 62 || px_pos == px_end {
                bytes[p] = QOI_OP_RUN | (run - 1);
                p+=1;
                run = 0;
            }
        } else {
            if run > 0 {
                bytes[p] = QOI_OP_RUN | (run -1);
                p+=1;
                run = 0;
            }

            let index_pos = qoi_color_hash(&px) % 64;
            // println!("{}", index_pos);
            if index[index_pos] == px {
                bytes[p] = QOI_OP_INDEX | index_pos as u8;
                p+=1;
            } else {
                index[index_pos] = px;
                if px.a == prev_px.a {
                    // gives same result as the one below
                    // let vr: i8 = (px.r - prev_px.r) as i8;
                    // let vg: i8 = (px.g - prev_px.g) as i8;
                    // let vb: i8 = (px.b - prev_px.b) as i8;
                    let vr = px.r.wrapping_sub(prev_px.r) as i8;
                    let vg = px.g.wrapping_sub(prev_px.g) as i8;
                    let vb = px.b.wrapping_sub(prev_px.b) as i8;
                    let vg_r: i8 = vr.wrapping_sub(vg); 
                    let vg_b: i8 = vb.wrapping_sub(vg);
                    
                    if  
                        vr > -3 && vr < 2 && 
                        vr > -3 && vr < 2 && 
                        vr > -3 && vr < 2 
                    {
                        bytes[p] = QOI_OP_DIFF | ((vr+2) << 4) as u8 | ((vg+2) << 2) as u8 | (vb+2) as u8;
                        p+=1;
                    } 
                    else if 
                        vg_r > -9 && vg_r < 8 &&
                        vg > -33 && vg < 32 &&
                        vg_b > -9 && vg_b < 8
                    {
                        bytes[p] = QOI_OP_LUMA | (vg + 32) as u8;
                        p+=1;
                        bytes[p] = ((vg_r + 8) << 4) as u8 |  (vg_b + 8) as u8;
                        p+=1;
                    }
                    else {
                        bytes[p] = QOI_OP_RGB;
                        bytes[p+1] = px.r;
                        bytes[p+2] = px.g;
                        bytes[p+3] = px.b;
                        p += 4;
                    }
                } else {
                    bytes[p] = QOI_OP_RGBA;
                    bytes[p+1] = px.r;
                    bytes[p+2] = px.g;
                    bytes[p+3] = px.b;  
                    bytes[p+4] = px.a;  
                    p += 5
                }
            }
        }
        prev_px = px;
    }

    for i in 0 .. QOI_PADDING.len() {
        bytes[p] = QOI_PADDING[i];
        p+=1;
    }

    println!{"sefsef{data:#?}"};
    
    let mut file = std::fs::File::create("image.qoi").unwrap();
    std::io::Write::write_all(&mut file, &bytes[..p]).unwrap();
}

pub fn test_encoding() {
    let res: u32 = 20;
    let size = res * res * 3;
    let mut data: Vec<u8> = (0..size).map(|x| (x + 3) as u8).collect();
    
    qoi_encode(
        data, 
        QoiDesc{
            width: res as u32,
            height: res as u32,
            channels: 3,
            colorspace: 1
        }
    );

}
