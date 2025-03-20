// main.rs
use std::fs::{read_to_string, write, create_dir_all};
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

use pulldown_cmark::{Parser as mdParser, Options, html};
use tera::{Tera, Context};
use clap::Parser;
use walkdir::WalkDir; // For recursive directory traversal

/// コマンドラインオプション
#[derive(Parser, Debug)]
struct Opt {
    /// Input folder path (contains .md files)
    /// Markdownファイルが入っている入力フォルダパス
    #[arg(short = 'i', long = "input", default_value = "content")]
    input: String,

    /// Output folder path (where .html files will be written)
    /// 出力先フォルダパス（HTMLファイルを配置する場所）
    #[arg(short = 'o', long = "output", default_value = "dist")]
    output: String,
}

// A small struct for navigation items
// ナビゲーション用のアイテムを管理する構造体
#[derive(Debug, serde::Serialize)]
struct NavItem {
    title: String,
    url: String,
    folder: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    // コマンドライン引数をパース
    let opt = Opt::parse();
    let input_dir = Path::new(&opt.input);
    let output_dir = Path::new(&opt.output);

    // Initialize Tera (load templates from "templates" folder)
    // テンプレートフォルダを指定して Tera を初期化
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error parsing templates: {}", e);
            std::process::exit(1);
        }
    };

    // Collect .md files recursively from input_dir using WalkDir
    // WalkDirを使用して入力ディレクトリから再帰的に .md ファイルを収集
    let mut md_files = Vec::new();
    for entry in WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension() == Some(OsStr::new("md")) {
            md_files.push(path.to_path_buf());
        }
    }

    // Build a list of NavItem for the navigation menu
    // ナビゲーションメニュー用のリストを作成
    let nav_items: Vec<NavItem> = md_files.iter().map(|path| {
        let relative_path = path.strip_prefix(input_dir).unwrap();
        let title = relative_path.file_stem().unwrap().to_string_lossy().to_string();
        let folder = relative_path.parent().map(|p| p.to_string_lossy().to_string());
        let mut url_path = relative_path.to_path_buf();
        url_path.set_extension("html");
        let url = url_path.to_string_lossy().replace("\\", "/");
        NavItem { title, url, folder }
    }).collect();

    // Process each .md file and generate corresponding .html in the output directory
    // 各 .md ファイルを処理し、出力ディレクトリ内に対応する .html を生成
    for path in &md_files {
        // Compute the relative path from input_dir
        let relative_path = path.strip_prefix(input_dir).unwrap();

        // Read Markdown content
        // Markdownファイルの内容を読み込む
        let markdown_input = read_to_string(path)?;

        // Convert Markdown to HTML
        // Markdown を HTML に変換
        let parser_options = Options::all();
        let parser = mdParser::new_ext(&markdown_input, parser_options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        // Prepare Tera context
        // Tera用のコンテキストを準備
        let mut context = Context::new();
        context.insert("site_name", "NCG (Neknaj Circuit Game) Tutorial");
        context.insert("title", &format!("{} - My Site", relative_path.with_extension("").to_string_lossy()));
        context.insert("content", &html_output);
        context.insert("nav_items", &nav_items);

        // Render with Tera
        // Tera でテンプレートをレンダリング
        let rendered_html = tera.render("base.html", &context)?;

        // Determine output file path: output_dir joined with relative_path (with extension changed to .html)
        // 出力ファイルパスを決定：出力ディレクトリに相対パスを結合し、拡張子を .html に変更
        let mut out_path = output_dir.join(relative_path);
        out_path.set_extension("html");

        // Create parent directory if needed
        // 必要に応じて親ディレクトリを作成
        if let Some(parent) = out_path.parent() {
            create_dir_all(parent)?;
        }

        // Write the rendered HTML
        // レンダリング結果をファイルに書き込み
        write(&out_path, rendered_html)?;
        println!("Processed: {:?}", path);
    }

    println!("All files processed! Output in: {:?}", output_dir);
    Ok(())
}
