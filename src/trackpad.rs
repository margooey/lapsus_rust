use macos_multitouch;

pub fn start_stream()
{
    for mut device in macos_multitouch::get_multitouch_devices()
    {
        // start sending touches to callback
        device.register_contact_frame_callback(| device: macos_multitouch::MTDeviceRef, data: &[macos_multitouch::Finger], timestamp: f64, frame: i32 |
        {
            println!("Device: 0x{:x}, Timestamp:\t {}, Frame: {}", device as i32, timestamp, frame);
            for f in data
            {
                println!("Frame {}:\t \
                    Angle {:.3},\t \
                    ellipse {:.2}x{:.2},\t \
                    position ({:.6},{:.6}),\t \
                    velocity ({:.5},{:.5}),\t \
                    ID {},\t state {}, Finger number {}, {}?,\t \
                    size {},\t {}?, [{}, {}]?, {}?",
                    f.frame,
                    f.angle * 90.0 / std::f32::consts::FRAC_PI_2,
                    f.major_axis, f.minor_axis,
                    f.normalized.pos.x, f.normalized.pos.y,
                    f.normalized.vel.x, f.normalized.vel.y,
                    f.identifier, f.state, f.finger_number, f.unknown0,
                    f.size, f.unknown1, f.unknown2[0], f.unknown2[1], f.unknown3);
            }
        }).unwrap(); // can fail if a callback is registered to the device already
    }
    
    println!("Press enter key to abort\n");
        
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok().expect("Failed to read line");
}