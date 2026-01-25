use clap::Parser;
use std::net::Ipv4Addr;

/// Apply a network mask to 255.255.255.255 or print all masks 0-32
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Mask value (0-32)
    mask: Option<u8>,
}

fn main() {
    let args = Args::parse();
    let ip = Ipv4Addr::new(255, 255, 255, 255);

    match args.mask {
        Some(mask) => match get_masked_ip(ip, mask) {
            Ok(result) => println!("{}", result),
            Err(msg) => eprintln!("{}", msg),
        },
        None => {
            // No argument: print all masks 0-32
            for mask in 0..=32 {
                if let Ok(result) = get_masked_ip(ip, mask) {
                    println!("{}", result);
                }
            }
        }
    }
}

// Returns masked IP as Ok(String) or Err(String) if mask invalid
fn get_masked_ip(ip: Ipv4Addr, mask: u8) -> Result<String, String> {
    if mask > 32 {
        Err("Mask value must be between 0 and 32".to_string())
    } else {
        Ok(format!("{}: {}", mask, apply_mask(ip, mask)))
    }
}

fn apply_mask(ip: Ipv4Addr, mask: u8) -> Ipv4Addr {
    if mask == 0 {
        return Ipv4Addr::new(0, 0, 0, 0);
    }
    let mask_u32 = if mask == 32 {
        u32::MAX
    } else {
        !((1u32 << (32 - mask)) - 1)
    };
    Ipv4Addr::from(u32::from(ip) & mask_u32)
}
