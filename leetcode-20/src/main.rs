struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len()%2 != 0 {return false;}
        let mut stack = vec!['0'];
        for i in s.chars(){
            match i{
                //入栈
                '{'|'('|'[' => {stack.push(i)},
                //出栈
                '}' => {if stack.pop().unwrap() != '{' {return false}}
                ')' => {if stack.pop().unwrap() != '(' {return false}}
                ']' => {if stack.pop().unwrap() != '[' {return false}}
                (_) => (),
            }
        }
        //都出栈后，stack还会剩下‘0’
        if stack.len() == 1 {return true;}
        else{return false;}
    }
}
fn main(){
    println!("leetcode-20");
}
