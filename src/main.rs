use std::collections::HashMap;
use std::io::{self, Write};

struct Scoreboard {
    players: HashMap<usize, String>,
    scores: HashMap<usize, i32>,
    next_id: usize,
}

impl Scoreboard {
    fn new() -> Self {
        Scoreboard {
            players: HashMap::new(),
            scores: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_player(&mut self, name: String) -> Result<usize, String> {
        // 验证玩家名称
        if name.is_empty() {
            return Err("玩家名称不能为空".to_string());
        }
        
        if name.len() > 20 {
            return Err("玩家名称过长，请限制在20个字符以内".to_string());
        }
        
        // 检查是否包含非法字符
        if name.chars().any(|c| c.is_control() || c == '\t' || c == '\n' || c == '\r') {
            return Err("玩家名称不能包含控制字符".to_string());
        }
        
        // 检查是否已存在同名玩家
        if self.players.values().any(|existing_name| existing_name == &name) {
            return Err(format!("玩家名称 '{}' 已存在，请使用不同的名称", name));
        }
        
        let id = self.next_id;
        self.players.insert(id, name);
        self.scores.insert(id, 0);
        self.next_id += 1;
        Ok(id)
    }

    fn update_scores(&mut self, winner_id: usize) -> Result<(), String> {
        if !self.players.contains_key(&winner_id) {
            return Err(format!("玩家序号 {} 不存在", winner_id));
        }

        // 胜出玩家 +1 分
        *self.scores.get_mut(&winner_id).unwrap() += 1;

        // 其他玩家 -1 分
        for (id, score) in self.scores.iter_mut() {
            if *id != winner_id {
                *score -= 1;
            }
        }

        Ok(())
    }

    fn display_scoreboard(&self) {
        println!("\n=== 积分榜 ===");
        println!("{:<4} {:<15} {:<6}", "序号", "玩家名称", "积分");
        println!("{}", "-".repeat(30));
        
        let mut sorted_players: Vec<_> = self.players.iter().collect();
        sorted_players.sort_by_key(|(id, _)| *id);
        
        for (id, name) in sorted_players {
            let score = self.scores.get(id).unwrap_or(&0);
            println!("{:<4} {:<15} {:<6}", id, name, score);
        }
        println!();
    }

    fn list_players(&self) {
        println!("\n=== 玩家列表 ===");
        let mut sorted_players: Vec<_> = self.players.iter().collect();
        sorted_players.sort_by_key(|(id, _)| *id);
        
        for (id, name) in sorted_players {
            println!("{}: {}", id, name);
        }
        println!();
    }
}

fn get_input(prompt: &str) -> Result<String, String> {
    print!("{}", prompt);
    if let Err(_) = io::stdout().flush() {
        return Err("输出缓冲区刷新失败".to_string());
    }
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => Err("输入流已结束 (EOF)".to_string()),
        Ok(_) => {
            let trimmed = input.trim().to_string();
            // 检查输入长度
            if trimmed.len() > 50 {
                Err("输入内容过长，请限制在50个字符以内".to_string())
            } else {
                Ok(trimmed)
            }
        }
        Err(_) => Err("读取输入失败，可能是由于输入流错误或中断".to_string()),
    }
}

fn get_input_safe(prompt: &str) -> String {
    loop {
        match get_input(prompt) {
            Ok(input) => return input,
            Err(e) => {
                eprintln!("输入错误: {}", e);
                eprintln!("请重试或按 Ctrl+C 退出程序。");
                // 如果是EOF或严重错误，退出程序
                if e.contains("EOF") || e.contains("输入流") {
                    eprintln!("程序将退出。");
                    std::process::exit(1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_player_validation() {
        let mut scoreboard = Scoreboard::new();
        
        // 测试空名称
        assert!(scoreboard.add_player("".to_string()).is_err());
        
        // 测试过长名称
        let long_name = "a".repeat(25);
        assert!(scoreboard.add_player(long_name).is_err());
        
        // 测试包含控制字符的名称
        assert!(scoreboard.add_player("test\n".to_string()).is_err());
        assert!(scoreboard.add_player("test\t".to_string()).is_err());
        
        // 测试正常名称
        assert!(scoreboard.add_player("张三".to_string()).is_ok());
        
        // 测试重复名称
        assert!(scoreboard.add_player("张三".to_string()).is_err());
    }
    
    #[test]
    fn test_update_scores() {
        let mut scoreboard = Scoreboard::new();
        let id1 = scoreboard.add_player("玩家1".to_string()).unwrap();
        let id2 = scoreboard.add_player("玩家2".to_string()).unwrap();
        
        // 测试不存在的玩家ID
        assert!(scoreboard.update_scores(999).is_err());
        
        // 测试正常更新
        assert!(scoreboard.update_scores(id1).is_ok());
        assert_eq!(*scoreboard.scores.get(&id1).unwrap(), 1);
        assert_eq!(*scoreboard.scores.get(&id2).unwrap(), -1);
    }
    
    #[test]
    fn test_input_length_validation() {
        // 测试输入长度验证逻辑
        let test_input = "a".repeat(60);
        assert!(test_input.len() > 50); // 验证测试数据确实超过限制
        
        // 测试玩家名称长度限制
        let mut scoreboard = Scoreboard::new();
        let long_name = "很长的玩家名称".repeat(5); // 创建超长名称
        assert!(scoreboard.add_player(long_name).is_err());
    }
}

fn main() {
    println!("欢迎使用游戏积分板系统！");
    println!("首先，请录入所有参与游戏的玩家名称。");
    
    let mut scoreboard = Scoreboard::new();
    
    // 录入玩家
    loop {
        let name = get_input_safe("请输入玩家名称（输入 'done' 完成录入）: ");
        
        if name.to_lowercase() == "done" {
            if scoreboard.players.is_empty() {
                println!("至少需要录入一个玩家！");
                continue;
            }
            break;
        }
        
        match scoreboard.add_player(name.clone()) {
            Ok(id) => {
                println!("玩家 '{}' 已添加，序号为: {}", name, id);
            }
            Err(e) => {
                println!("添加玩家失败: {}", e);
                continue;
            }
        }
    }
    
    println!("\n玩家录入完成！");
    scoreboard.list_players();
    scoreboard.display_scoreboard();
    
    // 游戏循环
    loop {
        println!("请选择操作:");
        println!("1. 记录游戏结果（输入胜出玩家序号）");
        println!("2. 查看积分榜");
        println!("3. 查看玩家列表");
        println!("4. 退出程序");
        
        let choice = get_input_safe("请输入选择 (1-4): ");
        
        match choice.as_str() {
            "1" => {
                scoreboard.list_players();
                let winner_input = get_input_safe("请输入胜出玩家的序号: ");
                
                // 增强数字输入验证
                if winner_input.is_empty() {
                    println!("输入不能为空！");
                    continue;
                }
                
                // 检查是否包含非数字字符
                if !winner_input.chars().all(|c| c.is_ascii_digit()) {
                    println!("请输入有效的正整数！");
                    continue;
                }
                
                match winner_input.parse::<usize>() {
                    Ok(winner_id) => {
                        // 检查数字范围
                        if winner_id == 0 {
                            println!("玩家序号必须大于0！");
                            continue;
                        }
                        
                        if winner_id > 1000 {
                            println!("玩家序号过大，请输入合理的序号！");
                            continue;
                        }
                        
                        match scoreboard.update_scores(winner_id) {
                            Ok(()) => {
                                println!("积分已更新！");
                                scoreboard.display_scoreboard();
                            }
                            Err(e) => println!("错误: {}", e),
                        }
                    }
                    Err(_) => println!("数字解析失败，请输入有效的数字！"),
                }
            }
            "2" => {
                scoreboard.display_scoreboard();
            }
            "3" => {
                scoreboard.list_players();
            }
            "4" => {
                println!("感谢使用游戏积分板系统！再见！");
                break;
            }
            _ => {
                println!("无效选择，请输入 1-4 之间的数字。");
            }
        }
    }
}
