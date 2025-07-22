use std::{
  fs::File,
  io::Write,
  process::Command,
};

fn main() -> std::io::Result<()> {
  let abc = std::time::Instant::now();

  let mut filelist = File::create("images.txt")?;
  for i in 0..=3 {
    writeln!(filelist, "file 'vv/_/0.png'")?;
    writeln!(filelist, "duration 0.016")?;
  }
  writeln!(filelist, "file 'vv/_/0.png'")?;

  let status = Command::new("ffmpeg").args(&["-f", "concat", "-safe", "0", "-i", "images.txt", "-vsync", "vfr", "-pix_fmt", "yuv420p", "-y", "output.mp4"]).status()?;

  if status.success() {
    println!("Video created successfully!");
  }
  else {
    eprintln!("FFmpeg failed");
  }

  println!("{}.", (std::time::Instant::now() - abc).as_millis());

  Ok(())
}
