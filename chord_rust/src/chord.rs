use std::{cell::RefCell, net::UdpSocket, rc::Rc};

#[derive(Debug, Clone)]
pub struct ChordNode {
    pub id: u16,
    pub finger_table: [Option<Rc<RefCell<ChordNode>>>; 8],
    pub predessor: Option<Rc<RefCell<ChordNode>>>,
    tem_nodes: Vec<Rc<RefCell<ChordNode>>>,
}

impl ChordNode {
    /// This function creates a new ChordNode with the given id and
    /// finger_table should be initialized with [None,None,None,None,None,None,None,None]
    pub fn new(id: u16, finger_table: [Option<Rc<RefCell<ChordNode>>>; 8]) -> ChordNode {
        ChordNode {
            id: id,
            finger_table: finger_table,
            predessor: None,
            tem_nodes: vec![],
        }
    }

    /// This function is used to stabilize the finger table of the chord node with the given child node,
    ///  here child node is the node same as the chord node that is used for stabilize (clone the node and pass it as an argument to this function)
    pub fn stablize(
        &mut self,
        child: Rc<RefCell<ChordNode>>,
        socketlistner: &UdpSocket,
        src_addr: &std::net::SocketAddr,
    ) {
        let arr = self.finger_table.clone();
        for i in 0..self.finger_table.len() {
            if arr[i].is_some() {
                let current_node = &arr[i];
                current_node
                    .clone()
                    .unwrap()
                    .borrow_mut()
                    .tem_nodes
                    .push(child.clone());
                self.predessor = ChordNode::recursive_check(
                    Rc::clone(current_node.as_ref().unwrap()),
                    self.id,
                    &mut self.finger_table,
                    &self.predessor,
                );
                socketlistner
                    .send_to(
                        serde_json::to_string(&ChordNode::parsedata(&self.finger_table, self.id))
                            .unwrap()
                            .as_bytes(),
                        src_addr,
                    )
                    .unwrap();
            }
        }
        self.predessor
            .clone()
            .unwrap()
            .borrow_mut()
            .tem_nodes
            .push(child.clone());
        self.predessor = ChordNode::recursive_check(
            Rc::clone(&self.predessor.as_ref().unwrap()),
            self.id,
            &mut self.finger_table,
            &self.predessor,
        );
        socketlistner
            .send_to(
                serde_json::to_string(&ChordNode::parsedata(&self.finger_table, self.id))
                    .unwrap()
                    .as_bytes(),
                src_addr,
            )
            .unwrap();

        for node in &self.tem_nodes {
            //if self.id==84{println!("{}",node.borrow().id);}
            node.clone().borrow_mut().tem_nodes.push(child.clone());
            self.predessor = ChordNode::recursive_check(
                node.clone(),
                self.id,
                &mut self.finger_table,
                &self.predessor,
            );
            socketlistner
                .send_to(
                    serde_json::to_string(&ChordNode::parsedata(&self.finger_table, self.id))
                        .unwrap()
                        .as_bytes(),
                    src_addr,
                )
                .unwrap();
        }

        self.tem_nodes = vec![];
    }

    /// This function stabilizes the finger table by checking the every know node and it is a private function
    fn recursive_check(
        current_node: Rc<RefCell<ChordNode>>,
        root_id: u16,
        finger_table: &mut [Option<Rc<RefCell<ChordNode>>>; 8],
        pre_node: &Option<Rc<RefCell<ChordNode>>>,
    ) -> Option<Rc<RefCell<ChordNode>>> {
        let rt_table = &current_node.borrow().finger_table;
        let current_node_clone = &Some(current_node.clone());
        let mut pre_node_new = pre_node.clone();
        let pre_node_clone = pre_node.clone();
        let current_node_predessor = &current_node.borrow().predessor;
        for j in 0..rt_table.len() + 3 {
            let nodes = if j == rt_table.len() {
                &pre_node_clone
            } else if j == rt_table.len() + 1 {
                current_node_predessor
            } else if j == rt_table.len() + 2 {
                current_node_clone
            } else {
                &rt_table[j]
            };

            if let Some(node_ref) = nodes.clone() {
                if node_ref.try_borrow().is_ok()
                    && ChordNode::distance(pre_node.clone().unwrap(), node_ref.clone(), root_id)
                    && node_ref.borrow().id != root_id
                {
                    pre_node_new = nodes.clone();
                }

                for i in 0..8 as u16 {
                    if node_ref.try_borrow().is_ok()
                        && ((2u16.pow(i as u32)) % 256
                            <= ChordNode::forward_distance(node_ref.borrow().id, root_id))
                        && node_ref.borrow().id != root_id
                        && node_ref.borrow().id != pre_node_clone.clone().unwrap().borrow().id
                    {
                        if let Some(finger_node) = &finger_table[i as usize] {
                            if ChordNode::forward_distance(finger_node.borrow().id, root_id)
                                > ChordNode::forward_distance(node_ref.borrow().id, root_id)
                            {
                                finger_table[i as usize] = nodes.clone();
                            }
                        } else {
                            finger_table[i as usize] = nodes.clone();
                        }
                    }
                }
            }
        }
        return pre_node_new;
    }

    /// This function is used when new node is added to the chord ring, it is used to update the finger table of the bootstrap node.
    pub fn bootstrap_node_addition(&mut self, new_node: &Rc<RefCell<ChordNode>>) {
        for i in 0..8 as u16 {
            if (2u16.pow(i as u32)) % 256
                <= ChordNode::forward_distance(new_node.borrow().id, self.id)
            {
                if let Some(finger_node) = &self.finger_table[i as usize] {
                    if ChordNode::forward_distance(finger_node.borrow().id, self.id)
                        > ChordNode::forward_distance(new_node.borrow().id, self.id)
                    {
                        self.finger_table[i as usize] = Some(Rc::clone(new_node));
                    }
                } else {
                    self.finger_table[i as usize] = Some(Rc::clone(new_node));
                }
            }
        }
        if self.predessor.is_none() {
            self.predessor = Some(Rc::clone(new_node));
        } else {
            if ChordNode::distance(
                self.predessor.clone().unwrap(),
                Rc::clone(new_node),
                self.id,
            ) && new_node.borrow().id != self.id
            {
                self.predessor = Some(Rc::clone(new_node));
            }
        }
    }

    /// This function is used to update the finger table of the newly added node using the finger table of bootstrap node.
    /// arguments for this fingertable of bootstrap node and clone of bootstrap node.
    /// one's the new node is added you have to call the three functions (bootstrap_node_addition, first_node, handle_predecessor)
    pub fn first_node(
        &mut self,
        finger_table: &[Option<Rc<RefCell<ChordNode>>>; 8],
        root_node: Rc<RefCell<ChordNode>>,
    ) {
        let root = &Some(Rc::clone(&root_node));
        for i in 0..finger_table.len() + 1 {
            let nodes: &Option<Rc<RefCell<ChordNode>>> = if i == finger_table.len() {
                root
            } else {
                &finger_table[i]
            };
            if let Some(node_ref) = nodes {
                for i in 0..8 as u16 {
                    if node_ref.try_borrow().is_ok()
                        && (2u16.pow(i as u32)) % 256
                            <= ChordNode::forward_distance(node_ref.clone().borrow().id, self.id)
                        && self.predessor.clone().unwrap().borrow().id != node_ref.borrow().id
                    {
                        if self.finger_table[i as usize].is_none()
                            || (ChordNode::forward_distance(
                                self.finger_table[i as usize].clone().unwrap().borrow().id,
                                self.id,
                            ) > ChordNode::forward_distance(node_ref.borrow().id, self.id)
                                && node_ref.borrow().id != self.id)
                        {
                            self.finger_table[i as usize] = Some(Rc::clone(&node_ref));
                        }
                    }
                }
            }
        }
    }
    /// This function finds a predessor for the newly added node.
    pub fn handle_predecessor(
        &mut self,
        finger_table: &[Option<Rc<RefCell<ChordNode>>>; 8],
        root_node: &Option<Rc<RefCell<ChordNode>>>,
    ) {
        self.predessor = root_node.clone();
        for node in finger_table {
            if let Some(node_dum) = node {
                if node_dum.try_borrow().is_ok() && node_dum.borrow().id != self.id {
                    if ChordNode::distance(
                        self.predessor.clone().unwrap(),
                        node_dum.clone(),
                        self.id,
                    ) && node_dum.borrow().id != self.id
                    {
                        self.predessor = node.clone();
                    }
                }
            }
        }
    }

    pub fn parsedata(data: &[Option<Rc<RefCell<ChordNode>>>; 8], root_id: u16) -> Vec<i16> {
        let mut vec: Vec<i16> = Vec::new();
        vec.push(root_id as i16);
        for i in 0..data.len() {
            if let Some(node) = &data[i] {
                vec.push(node.borrow().id as i16);
            } else {
                vec.push(-1);
            }
        }
        vec
    }

    /// This is the distace function to find the nearest predessor.
    fn distance(
        oldnode: Rc<RefCell<ChordNode>>,
        newnode: Rc<RefCell<ChordNode>>,
        rootnodeid: u16,
    ) -> bool {
        let node_distance = (rootnodeid + 256 - newnode.borrow().id) % 256; //,(newnode.as_ref().unwrap().id-rootnodeid+256)%256);
        let predessor_distance = (rootnodeid + 256 - oldnode.borrow().id) % 256; //,(oldnode.as_ref().unwrap().id-rootnodeid+256)%256);
        if node_distance < predessor_distance {
            return true;
        }
        false
    }

    /// This is the distance function for nodes in the fingertable.
    fn forward_distance(hash_id: u16, root_id: u16) -> u16 {
        (hash_id + 256 - root_id) % 256
    }
}
