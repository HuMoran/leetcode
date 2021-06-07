// 给定一个无序的整数数组，找到其中最长上升子序列的长度。
// 示例：
// ```
// 输入：[10, 9, 2, 5, 3, 7, 101, 18]
// 输出：4
// 解释：最长上升子序列是 [2, 3, 7, 101]，它的长度是 4。
// ```
// 说明：
//     - 可能会有多种最长上升子序列的组合，你只需要输出对应的长度即可。
//     - 你的算法时间复杂度应该为O(n²)，进阶能做到O(n logn)

fn main() {
    let input = vec![10, 9, 2, 5, 3, 7, 101, 18];
    let out = max_length_of_list(input);
    println!("{:?}", out);
}

// dp求解
fn max_length_of_list(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len <= 1 {
        return len as i32;
    }
    let mut dp: Vec<usize> = vec![1; len];
    for (index, num) in nums.iter().enumerate() {
        for j in 0..index {
            if *num > nums[j] {
                dp[index] = dp[index].max(dp[j] + 1);
            }
        }
    }
    dp.into_iter().max().unwrap() as i32
}
