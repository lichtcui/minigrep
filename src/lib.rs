//! # minigrep
//! 从文件中查询对应字段所在行
//!
//! `cargo run -- key_word target_file.txt`
use std::error::Error;
use std::{env, fs};

/// 查询配置
///
/// 从命令行及环境变量中获取
pub struct Config {
    /// 关键字 `args[1]`
    pub query: String,
    /// 文件路径 `args[2]`
    pub file_path: String,
    /// 忽略大小写 `IGNORE_CASE=1`
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        //! 从命令行及环境变量获取配置
        //!
        //! `let config = Config::build(env::args()).unwrap();`

        if args.size_hint().0 < 3 {
            return Err("not enough arguments");
        }

        args.next();

        let query = args.next().unwrap();
        let file_path = args.next().unwrap();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //! 根据配置进行查询
    //!
    //! `if let Err(e) = minigrep::run(config) {...}`

    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*!
        查询关键字所在行
        ```
        use minigrep::search;
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
        ```
    */

    contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*!
        查询关键字所在行（忽略大小写）
        ```
        use minigrep::search_case_insensitive;
        let query = "rUsT";
        let contents = "\
                Rust:
                safe, fast, productive.
                Pick three.
                Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
        ```
    */

    contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
