// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = Some(Box::new(ListNode::new(0)));

        // simple conditions
        match (&l1, &l2) {
            (Some(_), Some(_)) => (),
            _ => return root,
        }

        // actual adding
        let mut active_node = root.as_mut().unwrap();
        let mut node_1 = l1.as_ref();
        let mut node_2 = l2.as_ref();
        let mut sum: i32 = 0;

        loop {
            if node_1.is_some() {sum += node_1.unwrap().val};
            if node_2.is_some() {sum += node_2.unwrap().val};

            match (sum, &node_1, &node_2) {
                (0, None, None) => break,
                (val, _, _) => {
                    active_node.next = Some(Box::new(ListNode::new(val % 10)));
                    active_node = active_node.next.as_mut().unwrap();
                    sum = sum/10;
                }
            }

            // next in list
            node_1 = node_1.map_or(None, |node| node.next.as_ref());
            node_2 = node_2.map_or(None, |node| node.next.as_ref());
        }

        let check = root.as_ref().unwrap().next.is_some();
        if check {return root.unwrap().next;}
        else {return root}
    }
}


fn main() {
    Solution::add_two_numbers(None, None);
    println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn collect_list(node: Option<Box<ListNode>>) -> Vec<i32> {
        let mut active_node = node.unwrap();
        let mut out = Vec::<i32>::new();
        loop {
            out.push(active_node.val);
            match active_node.next {
                Some(val) => active_node = val,
                None => break,
            }
        }
        out
    }

    fn create_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
        let mut root = Box::new(ListNode::new(vals[0]));
        let mut active_node = &mut root;
        for i in 1..vals.len() {
            active_node.next = Some(Box::new(ListNode::new(vals[i])));
            active_node  = active_node.next.as_mut().unwrap();
        }
        Some(root)
    }

    #[test]
    fn test1() {
        let root_1 = create_list(vec!(2,4,3));
        let root_2 = create_list(vec!(5,6,4));
        let expected = create_list(vec!(7,0,8));

        let added = Solution::add_two_numbers(root_1, root_2);
        assert_eq!(collect_list(added), collect_list(expected));
    }

    #[test]
    fn test2() {
        let root_1 = create_list(vec!(9,9,9,9,9,9,9));
        let root_2 = create_list(vec!(9,9,9,9));
        let expected = create_list(vec!(8,9,9,9,0,0,0,1));

        let added = Solution::add_two_numbers(root_1, root_2);
        assert_eq!(collect_list(added), collect_list(expected));
    }
 
    #[test]
    fn test3() {
        let root_1 = create_list(vec!(0));
        let root_2 = create_list(vec!(0));
        let expected = create_list(vec!(0));

        let added = Solution::add_two_numbers(root_1, root_2);
        assert_eq!(collect_list(added), collect_list(expected));
    }
 
    #[test]
    fn test4() {
        let root_1 = create_list(vec!(1,6,0,3,3,6,7,2,0,1));
        let root_2 = create_list(vec!(6,3,0,8,9,6,6,9,6,1));
        let expected = create_list(vec!(7,9,0,1,3,3,4,2,7,2));

        let added = Solution::add_two_numbers(root_1, root_2);
        assert_eq!(collect_list(added), collect_list(expected));
    }
 
    #[test]
    fn test5() {
        let root_1 = create_list(vec!(0,8,8,8,8,2,9,3,1,1));
        let root_2 = create_list(vec!(0,9,1,5,5,5,1,1,6));
        let expected = create_list(vec!(0,7,0,4,4,8,0,5,7,1));

        let added = Solution::add_two_numbers(root_1, root_2);
        assert_eq!(collect_list(added), collect_list(expected));
    }
 
    #[test]
    fn test6() {
        let root_1 = create_list(vec!(3,6,0,1,1,5,8,3));
        let root_2 = create_list(vec!(0,3,0,9,2,1,4,0,0,1));
        let expected = create_list(vec!(3,9,0,0,4,6,2,4,0,1));

        let added = Solution::add_two_numbers(root_1, root_2);
        assert_eq!(collect_list(added), collect_list(expected));
    }
}
