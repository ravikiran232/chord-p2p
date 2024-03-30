
pub mod chord;

use chord::ChordNode;
use crossbeam::channel::{unbounded, Receiver, Sender};
use std::cell::RefCell;
use std::net::UdpSocket;
use std::rc::Rc;
fn main() {
    let (s, r): (Sender<Vec<i16>>, Receiver<Vec<i16>>) = unbounded();

    let (s1, r1): (Sender<String>, Receiver<String>) = unbounded();

    let handle = std::thread::spawn(move || {

        let listner = UdpSocket::bind("127.0.0.1:8080").unwrap();
        loop {
            let mut buffer = [0; 1024];
            let (number_of_bytes, src_addr) = listner.recv_from(&mut buffer).unwrap();
            let message = String::from_utf8_lossy(&buffer[..number_of_bytes]).into_owned();
            s1.send(message).unwrap();
            //listner.send_to("hello buddy".as_bytes(), src_addr).unwrap();
            loop {
                // if let Ok(data)=r.try_recv(){
                let data = r.recv().unwrap();
                if data.len() == 0 {
                    break;
                } else {
                    listner
                        .send_to(serde_json::to_string(&data).unwrap().as_bytes(), src_addr)
                        .unwrap();
                }
            }
        }
    });

    // println!("Welcome to the chord viz you can see the finger tables in the fingertable.txt (it will be updated realtime)");
    let mut my_nodes: Vec<Rc<RefCell<ChordNode>>> = Vec::new();
    let bootstrap_node: ChordNode =
        ChordNode::new(1, [None, None, None, None, None, None, None, None]);
    my_nodes.push(Rc::new(RefCell::new(bootstrap_node.clone())));
    let mut count: u8 = 1;
    loop {
        // let mut input:String=String::from("");
        // println!("Enter your node id or type exit to quit the programm ");
        // io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = r1.recv().unwrap();
        if input.trim() == "exit" {
            break;
        } else {
            let finger_table: [Option<Rc<RefCell<ChordNode>>>; 8] =
                [None, None, None, None, None, None, None, None];
            let id: u16 = input.trim().parse().unwrap();

            let new_node: Rc<RefCell<ChordNode>> =
                Rc::new(RefCell::new(ChordNode::new(id, finger_table)));

            my_nodes[0]
                .borrow_mut()
                .bootstrap_node_addition(&Rc::clone(&new_node));
            new_node.borrow_mut().handle_predecessor(
                &my_nodes[0].borrow().finger_table,
                &Some(Rc::clone(&my_nodes[0])),
            );
            new_node
                .borrow_mut()
                .first_node(&my_nodes[0].borrow().finger_table, my_nodes[0].clone());

            my_nodes.push(new_node);
        }

        let mut count1 = 0;
        while count1 < 3 {
            for node in &my_nodes {
                node.borrow_mut().stablize(node.clone(), &s);
                s.send(ChordNode::parsedata(
                    &node.borrow().finger_table,
                    node.borrow().id,
                ))
                .unwrap();
            }
            count1 += 1;
        }
        s.send(vec![]).unwrap();

        //  to be used if you want to write the finger table to a file

        // let mut file = std::fs::OpenOptions::new()
        //     .write(true)
        //     .append(true)
        //     .create(true)
        //     .open(format!("fingertable{}.txt", count))
        //     .unwrap();

        // for i in 0..my_nodes.len() {
        //     writeln!(
        //         file,
        //         "{}\n",
        //         format!("finger table of node {:?}", my_nodes[i].borrow().id)
        //     )
        //     .expect("unable to write");
        //     if my_nodes[0].borrow().predessor.is_some() {
        //         writeln!(
        //             file,
        //             " the node predessor is {}",
        //             my_nodes[i].borrow().predessor.clone().unwrap().borrow().id
        //         )
        //         .expect("unable to write");
        //     } else {
        //         writeln!(file, "predessor is None").expect("unable to write");
        //     }
        //     for node in &my_nodes[i].borrow().finger_table {
        //         if node.is_some() {
        //             writeln!(file, "{:?}\n", node.clone().unwrap().borrow().id)
        //                 .expect("unable to write");
        //         } else {
        //             writeln!(file, "None").expect("msg");
        //         }
        //     }
        //     writeln!(
        //         file,
        //         "{}\n",
        //         format!("finger table of node {:?} ended", my_nodes[i].borrow().id)
        //     )
        //     .expect("unable to write");
        // }
        count += 1;
    }

    handle.join().unwrap();
}
