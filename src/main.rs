use base64::{engine::general_purpose::STANDARD, Engine};
use libaes::Cipher;
use regex;
use reqwest;
use scraper::{ElementRef, Html, Selector};
use serde_json::{self, Value};
use text_io::read;
fn encrypt(plain: &str) -> String {
    let iv: [u8; 16] = [0, 2, 7, 3, 5, 3, 8, 0, 0, 117, 110, 105, 118, 84, 77, 87];
    let key = b"zbcd1efghi4jklm2nopq5rstu3vw6xya";
    let cipher = Cipher::new_256(key);

    let out = cipher.cbc_encrypt(&iv, plain.as_bytes());
    STANDARD.encode(out).replace("+", "~univ~")
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let plain: &str = "Nickname=ㅇㄴㄹ&Q1=P&Q2=F&Q3=S&Q4=F&Q5=P&Q6=N&Q7=S&Q8=P&Q9=F";
    let title = "당신의 이름 또는\n닉네임을 알려주세요.";
    let test_title = [
        "입사 후 맡게 된 첫 프로젝트!\n뭐부터 시작하면 좋을까?",
        "오랜만에 만난 입사 동기,\n\n힘들다고 고민 상담을 한다면?\n",
        "나는 이런 사람이고 싶어!\n팀 내에서 맡고 싶은 포지션은?\n",
        "프로젝트를 진행하면서 어려움에 처했다면!?\n\n어떻게 하는 게 좋을까?\n",
        "드디어 찾아온 주말,\n\n어떻게 하루를 보내면 좋을까?\n",
        "신사업 아이디어 회의,\n자료를 어떻게 준비해야 할까?\n",
        "입사한 지 어느덧 1년!\n\n돌아보면 무엇이 가장 뿌듯할까?\n",
        "급히 보고서를 마무리해야 하는데,\n\n타팀에서 업무 협조 요청이 왔다면?\n",
        "아이디어 회의 중,\n\n팀원들 간의 의견이 분분하다면?\n",
    ];
    let form_label = [
        "이거부터 해야겠네!할 수 있는 일부터 차근차근 해보기",
        "한 치의 오차도 없게! 해야 할 업무 리스트업, 체크리스트 만들기",
        "많이 힘들었겠구나…\n\n공감해 주고 따듯하게 위로해 주기\n",
        "아하, 왜 힘들었는데?\n이유를 들어보고 해결책 제시해주기\n",
        "묵묵하고 꼼꼼하게 맡은 업무를 해내는 사람\n",
        "유연하게 새로운 아이디어를 제시하는 사람\n",
        "다 이렇게 성장하는 거지~\n\n긍정적으로 마인드 셋 하며 동료에게 도움 요청하기\n",
        "어떤 부분이 문제였을까?\n\n곰곰이 생각해 보면서 상사에게 자문구하기\n",
        "무계획이 계획!\n\n집 근처에서 발길 닿는 대로 구경하기\n",
        "운동도 해야 하고…또 뭐 해야 했더라?\n\n평일에 하지 못했던 것 처리하기\n",
        "최대한 다양한 방향으로\n\n아이디어를 구상하고 자료 스크랩해 가기\n",
        "가장  현실성있고 좋은 아이디어를\n\n디벨롭해서 문서화 해가기\n",
        "성실한게 최고! 맡은 업무를 꾸준하게 수행한 점\n",
        "새롭게 배우면서  조직에 적응해 나가는 점\n",
        "바로  전달가능한 자료를 취합해서 먼저 전달한다.\n",
        "먼저 양해를 구하고, 보고서 마무리 후 협조한다.\n",
        "모두의 의견을 차례대로 들어보고 과열된 분위기를 중재한다.\n",
        "가장 합리적인 안으로 의견이 합치될 수 있도록 제안한다.\n",
    ];
    let form_radio = [
        'P', 'J', 'F', 'T', 'S', 'N', 'F', 'T', 'P', 'J', 'N', 'S', 'S', 'N', 'P', 'J', 'F', 'T',
    ];

    let mut answers = vec![];

    println!("{}", title);
    let nick_name: String = read!();
    answers.push(format!("Nickname={}", nick_name));

    for i in 0..test_title.len() {
        loop {
            println!("{}", test_title.get(i).unwrap());
            println!("1) {}", form_label.get(2 * i).unwrap());
            println!("2) {}", form_label.get(2 * i + 1).unwrap());
            print!("> ");
            let input: usize = read!("{}");

            if input == 1 || input == 2 {
                answers.push(format!(
                    "Q{}={}",
                    i + 1,
                    form_radio.get((2 * i) + input - 1).unwrap()
                ));
                break;
            }
        }
    }

    let base_url = "https://www.hyundaijobfair2023.com";
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{base_url}/CodeTest/TestSave"))
        .body(format!("p={}", encrypt(&answers.join("&"))))
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8",
        )
        .send()
        .await
        .unwrap();
    match resp.text().await.unwrap() {
        resp => {
            let res: Value = serde_json::from_str(&resp).unwrap();
            if res["success"] == true {
                let href = &res["href"].as_str().unwrap().trim_matches('"');
                let resp = client
                    .get(format!("{base_url}{href}"))
                    .send()
                    .await
                    .unwrap();
                let data = resp.text().await.unwrap();
                let document = Html::parse_document(&data.as_str());
                let script_sel = Selector::parse("script").unwrap();
                let script_content: String = document
                    .select(&script_sel)
                    .nth(4)
                    .unwrap()
                    .text()
                    .collect();

                let re = regex::Regex::new(r"`.+`").unwrap();
                let mut result = vec![];
                re.find_iter(&script_content).for_each(|x| {
                    result.push(x.as_str().trim_matches('`'));
                });
                println!("{}\n", result.join(" "));

                let hash_sel = Selector::parse(".hash").unwrap();
                let hash_content = document
                    .select(&hash_sel)
                    .nth(0)
                    .unwrap()
                    .text()
                    .map(|x| x.trim())
                    .fold(vec![], |mut acc: Vec<_>, cur: &str| {
                        acc.push(cur);
                        acc
                    })
                    .join(" ");
                println!("{}\n", hash_content);

                let title_sel = Selector::parse("strong.title").unwrap();
                let title_content: String = document
                    .select(&title_sel)
                    .nth(0)
                    .unwrap()
                    .text()
                    .collect::<String>()
                    .split_whitespace()
                    .map(|x| format!("{x} "))
                    .collect();
                println!("☝ {title_content}");

                let info_list_sel = Selector::parse(".info-list").unwrap();
                let info_list_content = document
                    .select(&info_list_sel)
                    .nth(0)
                    .unwrap()
                    .children()
                    .filter_map(|child| ElementRef::wrap(child))
                    .map(|e| {
                        let desc = e
                            .text()
                            .collect::<String>()
                            .split_whitespace()
                            .collect::<Vec<_>>()
                            .join(" ");
                        format!("- {desc}")
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                println!("{}", info_list_content);

                let title_content: String = document
                    .select(&title_sel)
                    .nth(1)
                    .unwrap()
                    .text()
                    .collect::<String>()
                    .split_whitespace()
                    .map(|x| format!("{x} "))
                    .collect();

                println!("");
                println!("⭐ {title_content}");

                let intro_box_sel = Selector::parse(".intro-box").unwrap();
                let intro_box_content = document
                    .select(&intro_box_sel)
                    .nth(0)
                    .unwrap()
                    .children()
                    .filter_map(|child| ElementRef::wrap(child))
                    .map(|e| e.text().collect::<String>())
                    .collect::<Vec<_>>()
                    .join(" : ");

                println!("- {intro_box_content}");

                let description_sel = Selector::parse(".description").unwrap();
                let description_content = document
                    .select(&description_sel)
                    .nth(0)
                    .unwrap()
                    .text()
                    .collect::<String>()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
                println!("  {}\n", description_content);

                let title_content: String = document
                    .select(&title_sel)
                    .nth(2)
                    .unwrap()
                    .text()
                    .collect::<String>()
                    .split_whitespace()
                    .map(|x| format!("{x} "))
                    .collect();
                println!("🤝 {title_content}");

                let intro_box_content = document
                    .select(&intro_box_sel)
                    .nth(1)
                    .unwrap()
                    .children()
                    .nth(1)
                    .unwrap()
                    .children()
                    .filter_map(|child| ElementRef::wrap(child))
                    .map(|e| e.text().collect::<String>())
                    .collect::<String>()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" : ");

                println!("{intro_box_content}");
            }
        }
        _ => println!("‼ 서버로부터 데이터를 전송받지 못했습니다"),
    }
}
