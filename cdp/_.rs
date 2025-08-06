fn main() -> std::io::Result<()> {
  let mut child = Command::new(r"C:\Program Files\Google\Chrome\Application\chrome.exe").args(["--user-data-dir=C:/chrome-profile-debug", "--no-first-run", "--no-default-browser-check"]).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;

  let stdin = child.stdin.as_mut().unwrap();
  let stdout = child.stdout.take().unwrap();
  let mut reader = BufReader::new(stdout);

  // Send CDP command: Target.getBrowserContexts
  let cmd = CdpCommand {
    id: 1,
    method: "Network.enable",
  };

  let cmd_json = serde_json::to_string(&cmd)? + "\n";
  stdin.write_all(cmd_json.as_bytes())?;
  stdin.flush()?;

  // Read Chrome's response
  //   let mut response_line = String::new();
  //   reader.read_line(&mut response_line)?;

  //   println!("{}", response_line);

  let mut line = String::new();
  loop {
    line.clear(); // important: reuse the buffer
    let bytes = reader.read_line(&mut line)?;
    if bytes == 0 {
      // EOF or Chrome exited
      break;
    }

    println!("Raw response: {}", line.trim_end());

    // Optional: parse as CDP JSON
    // if let Ok(response) = serde_json::from_str::<CdpResponse>(&line) {
    //   println!("Parsed CDP Response: {:#?}", response);
    // }
    // else {
    //   eprintln!("Failed to parse line: {}", line.trim_end());
    // }
  }

  //   let response: CdpResponse = serde_json::from_str(&response_line)?;
  //   println!("CDP Response: {:#?}", response);

  loop {
    std::thread::sleep(Duration::from_millis(1));
  }
}

#[derive(Serialize)]
struct CdpCommand<'a> {
  id: u32,
  method: &'a str,
}

#[derive(Deserialize, Debug)]
struct CdpResponse {
  id: u32,
  result: serde_json::Value,
}

use {
  serde::{
    Deserialize,
    Serialize,
  },
  std::{
    io::{
      BufRead,
      BufReader,
      Write,
    },
    process::{
      Command,
      Stdio,
    },
    time::Duration,
  },
};
