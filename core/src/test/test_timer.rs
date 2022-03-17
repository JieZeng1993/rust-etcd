// #[test]
// fn test_timer() {
//     use std::sync::mpsc::channel;
//
//     let timer = timer::Timer::new();
//     let (tx, rx) = channel();
//
//     timer.schedule_with_delay(chrono::Duration::seconds(3), move || {
//         tx.send(()).unwrap();
//     });
//
//     rx.recv().unwrap();
//     println!("This code has been executed after 3 seconds");
// }