use owo_colors::OwoColorize;

use crate::{repl::style, tools::Tool};

pub fn tools() -> Vec<Tool> {
    vec![Tool {
        name: "tools::tcp::ping".to_string(),
        description: "Check whether a host is reachable".to_string(),
        usage: "tools::tcp::ping <host> <port> <retries>".to_string(),
        entry_point: |args| {
            let host = args.get(0).ok_or_else(|| anyhow::anyhow!("Missing host argument"))?;
            let port = args.get(1).ok_or_else(|| anyhow::anyhow!("Missing port argument"))?.parse::<u16>()?;
            let retries = args.get(2).ok_or_else(|| anyhow::anyhow!("Missing retries argument"))?.parse::<u32>()?;
            let host = host.clone();

            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async move {
                    crate::tools::tcp::ping::ping(&host, port, retries).await
                })
            })
        },
    },
    Tool {
        name: "tools::tcp::reverse_tcp".to_string(),
        description: "Set up a reverse TCP shell".to_string(),
        usage: "tools::tcp::reverse_tcp <lport>".to_string(),
        entry_point: |args| {
            let lport = args.get(0).ok_or_else(|| anyhow::anyhow!("Missing local port argument"))?.parse::<u16>()?;

            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async move {
                    crate::tools::tcp::reverse_tcp::reverse_tcp(lport).await
                })
            })
        },
    },
    Tool {
        name: "tools::encode".to_string(),
        description: "Encode a string in various formats".to_string(),
        usage: "tools::encode <b64/hex/rot13> <input>".to_string(),
        entry_point: |args| {
            let result = crate::tools::encode::encode_str(&args)?;
            println!("{}", result);
            Ok(())
        },
    },
    Tool {
        name: "tools::decode".to_string(),
        description: "Decode a string from various formats".to_string(),
        usage: "tools::decode <b64/hex/rot13> <input>".to_string(),
        entry_point: |args| {
            let result = crate::tools::encode::decode_str(&args)?;
            println!("{}", result);
            Ok(())
        },
    },
    Tool {
        name: "tools::recon::dns".to_string(),
        description: "Resolve DNS records for a domain".to_string(),
        usage: "tools::recon::dns <domain>".to_string(),
        entry_point: |args| {
            let domain = args.get(0).ok_or_else(|| anyhow::anyhow!("Missing domain argument"))?.clone();

            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async move {
                    crate::tools::recon::dns::dns(&domain).await
                })
            })
        },
    },
    Tool {
        name: "tools::http::get".to_string(),
        description: "Send an HTTP GET request".to_string(),
        usage: "tools::http::get <url> [key=value ...]".to_string(),
        entry_point: |args| {
            let url = args.get(0).ok_or_else(|| anyhow::anyhow!("Missing url argument"))?.clone();
            let params = parse_query_pairs(&args[1..])?;
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async move {
                    let borrowed = params.as_ref().map(|pairs| {
                        pairs
                            .iter()
                            .map(|(key, value)| (key.as_str(), value.as_str()))
                            .collect::<Vec<_>>()
                    });
                    crate::tools::http::get::url(&url, borrowed.as_deref()).await
                })
            })
        },
    },
    Tool {
        name: "tools::http::post".to_string(),
        description: "Send an HTTP POST request".to_string(),
        usage: "tools::http::post <url> [--data body] [key=value ...]".to_string(),
        entry_point: |args| {
            let (url, params, body) = parse_http_args(&args)?;
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async move {
                    let borrowed = params.as_ref().map(|pairs| {
                        pairs
                            .iter()
                            .map(|(key, value)| (key.as_str(), value.as_str()))
                            .collect::<Vec<_>>()
                    });
                    crate::tools::http::post::url(&url, borrowed.as_deref(), body.as_deref()).await
                })
            })
        },
    },
    Tool {
        name: "tools::http::put".to_string(),
        description: "Send an HTTP PUT request".to_string(),
        usage: "tools::http::put <url> [--data body] [key=value ...]".to_string(),
        entry_point: |args| {
            let (url, params, body) = parse_http_args(&args)?;
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async move {
                    let borrowed = params.as_ref().map(|pairs| {
                        pairs
                            .iter()
                            .map(|(key, value)| (key.as_str(), value.as_str()))
                            .collect::<Vec<_>>()
                    });
                    crate::tools::http::put::url(&url, borrowed.as_deref(), body.as_deref()).await
                })
            })
        },
    },
    Tool {
        name: "tools::http::delete".to_string(),
        description: "Send an HTTP DELETE request".to_string(),
        usage: "tools::http::delete <url> [key=value ...]".to_string(),
        entry_point: |args| {
            let url = args.get(0).ok_or_else(|| anyhow::anyhow!("Missing url argument"))?.clone();
            let params = parse_query_pairs(&args[1..])?;
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async move {
                    let borrowed = params.as_ref().map(|pairs| {
                        pairs
                            .iter()
                            .map(|(key, value)| (key.as_str(), value.as_str()))
                            .collect::<Vec<_>>()
                    });
                    crate::tools::http::delete::url(&url, borrowed.as_deref()).await
                })
            })
        },
    }
    ]
}

pub fn run_tool(tool: &Tool, args: Vec<String>) -> anyhow::Result<()> {
    (tool.entry_point)(args)
}

pub fn run_tool_by_path(path: &[String], args: Vec<String>) -> anyhow::Result<()> {
    let name = path.join("::");
    let tool = tools().into_iter().find(|t| t.name == name).ok_or_else(|| anyhow::anyhow!("Tool not found: {name}"))?;
    run_tool(&tool, args)
}

pub fn print_help() {
    println!("{}", "Available commands:".bold());
    for tool in tools() {
        println!("  {} - {}", style::color_namespace_path(&tool.name), tool.description);
        println!("      {}", style::color_namespace_path(&tool.usage));
    }
}

fn parse_query_pairs(args: &[String]) -> anyhow::Result<Option<Vec<(String, String)>>> {
    let mut pairs = Vec::new();

    for arg in args {
        if let Some((key, value)) = arg.split_once('=') {
            pairs.push((key.to_string(), value.to_string()));
        }
    }

    if pairs.is_empty() {
        Ok(None)
    } else {
        Ok(Some(pairs))
    }
}

fn parse_http_args(args: &[String]) -> anyhow::Result<(String, Option<Vec<(String, String)>>, Option<String>)> {
    let url = args.get(0).ok_or_else(|| anyhow::anyhow!("Missing url argument"))?.clone();
    let mut params = Vec::new();
    let mut body = None;
    let mut index = 1;

    while index < args.len() {
        if args[index] == "--data" || args[index] == "-d" {
            index += 1;
            let value = args.get(index).ok_or_else(|| anyhow::anyhow!("Missing body after --data"))?;
            body = Some(value.clone());
        } else if let Some((key, value)) = args[index].split_once('=') {
            params.push((key.to_string(), value.to_string()));
        } else {
            body = Some(args[index].clone());
        }

        index += 1;
    }

    let params = if params.is_empty() { None } else { Some(params) };
    Ok((url, params, body))
}