# 🕸 Filaris

`filaris` is a fast multithreaded tool for exploring and trace pathways within any website.

<div> 
  <img src="https://github.com/user-attachments/assets/41a64a98-8994-459d-8955-4ba27b3b662f" alt="screenshot" />
</div>


<!-- Getting Started -->
## 	:toolbox: Getting Started

<!-- Prerequisites -->
### :bangbang: Prerequisites

```bash
 sudo apt install libssl-dev
```

<!-- Run Locally -->
### :running: Run Locally

Clone the project

```bash
  git clone https://github.com/YuriRDev/filaris
```

Go to the project directory

```bash
  cd filaris
```

🌟 Run Filaris

```bash
  cargo run --url "yourwebsite.com"
```

<!-- Usage -->
## :eyes: Usage


#### Table of Args

| Name | Value | Default | Description |
|------|-------|---------|-------------|
| `url` | String | - | The initial URL to start scanning from. |
| `max_urls` | Integer | `1000` | Specifies the maximum number of URL to discover. |
| `match_str` | String | `""` |  A string that new URLs must contain to be considered. |
| `ignore` | Vec<String> | `[]` | URLs containing any of these strings will be ignored. |
| `concurrency` | Integer | `10` | Number of tasks that will be spawned concurrently. |

```bash
  cargo run --url "yourwebsite.com" --match-str "yourwebsite.com" --ignore "wordpress" --ignore "wp" --concurrency 10
```

<!-- Roadmap -->
## :compass: Roadmap
Here, are some known bugs and WIP, all the items here should be resolved in a few days. 

* [ ] Don't add to queue items that are already have been scanned - That causes loops.
* [x] Don't add to queue items that are already in queue - That causes loops.
* [ ] Usage of Priority Queue instead of a simple Queue.
* [ ] Bring back `--depth` arg
