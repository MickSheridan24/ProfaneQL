

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
struct  Solution {

}

impl Solution {

    pub fn add_next(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>,  last_plus: i32, init: bool) -> Option<Box<ListNode>> {
        let mut val = last_plus;
        let mut next_plus = 0;  


        let mut next_node = None; 
        let cont = l1.is_some() || l2.is_some();

        let l1_curr = l1.unwrap_or(Box::new(ListNode::new(0)));
        let l2_curr = l2.unwrap_or_else(|| Box::new(ListNode::new(0)));


        val = val + l1_curr.val + l2_curr.val;

        if val >= 10 {
            val = val - 10;
            next_plus = 1; 
        }

        print!("{0}\n", l1_curr.val);
        print!("{0}\n", l2_curr.val);
        print!("{0}\n", val);
        print!("{0}\n\n", next_plus);


        if cont {
            next_node = Self::add_next(l1_curr.next, l2_curr.next, next_plus, false)
        }


        if cont || val > 0 || next_plus == 1 || init {
            let next = Some(Box::new(ListNode{
                val: val, 
                next: next_node
            }));
            return next; 
        }
        else {
            return  None; 
        }

    }



    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {



        return Self::add_next(l1, l2, 0, true);

    
    }

}

