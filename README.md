> If you see this section, you've just created a repository using [PoC Innovation's Open-Source project template](https://github.com/PoCInnovation/open-source-project-template). Check the [getting started guide](./.github/getting-started.md).


# Kermit CLI

Kermit is a powerful, user-friendly command-line interface (CLI) designed for Alephium to interact with a blockchain node and manage wallets. Whether you're a developer, node operator, or enthusiast, Kermit provides essential utilities to manage blockchain nodes, perform wallet operations, monitor mempool transactions, and retrieve various node metrics.

⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡴⠖⠛⠛⠛⠛⠳⢯⣁⣀⣤⡴⠦⣥⣀⣬⣷⠶⠭⣥⣄⡀⣠⡼⢤⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⠋⠀⣠⣶⣶⣶⠶⠂⠀⠹⣯⠀⢠⣤⣤⠉⠁⠀⣀⣀⠀⠈⠙⢻⣷⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⠁⠀⣠⡿⠟⠋⠀⠀⢀⣠⡶⠛⠀⠀⠀⠹⣦⡀⠀⠈⢻⣿⣷⣤⠀⠹⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣸⣷⠀⠀⣉⠀⠀⣀⣠⠴⠛⠁⠀⡀⠤⠑⠀⡀⠈⠳⣄⠀⠀⠙⢻⣿⠀⠀⣿⣷⡄⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⢀⡤⠞⠋⠁⠘⠳⠶⠶⠶⠛⠋⡁⠠⠐⠤⢁⠂⠀⠀⠀⠀⠀⠀⠉⠳⣤⣀⠈⠋⠀⣰⣿⣸⡇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⣰⠏⣠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⢀⠒⠠⠘⡀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠉⠓⠶⠖⠛⢿⣯⣦⣀⣤⣀⣄⣀⡀⠀
⠀⠀⠀⠀⠀⠀⢸⠇⣸⣿⡄⠀⠠⠄⠁⠀⠀⠀⠀⠀⡈⠄⠒⡠⠐⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢳⣌⣷⠀⢿⠛⠻⡅
⠀⠀⠀⠀⠀⠀⢺⡄⡹⢾⣷⡀⠀⠀⠀⠀⠀⠀⠀⠌⠀⢂⠁⢂⠁⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⢻⣿⠀⢸⡇⠀⡇
⠀⠀⠀⠀⠀⠀⢸⡇⢼⡹⣟⣿⡄⠰⣬⣡⠂⢒⠠⢌⠂⡌⠢⠁⢀⠐⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⡧⣾⣿⠀⠀⠇⠀⣇
⠀⠀⠀⠀⠀⠀⠀⢹⡜⣵⢫⢟⣷⡀⠀⠙⠻⣦⣶⣬⣄⣂⡐⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⡀⣠⠴⠛⢫⢵⣿⠧⠀⠀⣸⠀⡿
⠀⠀⠀⠀⠀⠀⢀⣠⣿⡜⠦⢋⠞⢷⠀⠀⡠⢄⡉⠝⡋⠟⠿⠿⠷⠷⠶⠶⠾⠞⠛⠛⠛⠋⠉⠀⠀⠀⠠⠌⡑⢂⣿⢯⠃⠀⠀⠃⢀⣇
⠀⢀⣀⣤⠶⠋⠁⣀⣤⣷⡄⠀⢎⡘⢦⠰⣱⢢⢉⠮⡑⢮⠰⣀⠆⢀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⣈⣤⡿⠃⠀⢀⠆⠀⠀⣼⡍
⠚⠋⢁⣠⠤⢒⢾⡻⣽⢯⣿⡄⠺⣄⠣⡙⢆⡣⢞⡰⡙⢦⢣⢍⡚⢆⢳⠘⡄⠣⠉⠆⢀⡀⢀⠀⢀⢀⣷⠟⠉⠀⢀⡴⠋⠀⠀⢰⡏⠀
⠠⠒⠉⢠⠒⣭⠶⡽⣧⢿⣿⣇⢶⣌⢡⠓⡬⢜⣣⠲⡙⢦⡉⠶⡘⢆⡣⣉⠘⣅⠚⡌⢂⠠⣁⣬⣾⠟⠃⠀⢀⠒⠈⠀⠀⣠⡴⠋⠀⠀
⠀⠀⢈⠂⣋⠶⣹⢹⡞⣯⢿⣿⣌⠿⣧⡑⠘⡆⢧⡙⡝⢦⡙⡤⣙⠢⢥⢨⡑⢢⡙⢀⣣⣶⣿⡟⠁⡀⢄⣰⣦⣋⣤⡶⠛⠁⠀⠀⠀⠀
⠤⠶⠲⢞⣲⠶⠃⡓⢞⡵⣻⣿⣧⢫⠽⣿⣌⠘⢤⠹⡜⣡⠞⡰⡡⢞⢢⡱⡌⠧⣘⣾⣿⢫⣿⠀⠂⠄⠸⢹⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀
⠀⡴⣩⠞⠁⠀⠀⠌⣈⠲⢍⣿⣿⣌⠫⣝⡻⣧⢀⠫⡜⢤⠋⣖⡙⢦⢃⢖⠩⡒⣿⠋⡞⢿⡿⠀⠄⠀⠀⠀⢻⡆⠀⠀⠀⠀⠀⠀⠀⠀
⠴⠋⠀⠀⠀⠀⠀⠀⠀⠀⠈⡘⢿⣧⡓⣌⠳⣙⢧⢣⠘⡥⢚⠤⡹⢌⡩⢌⣢⣿⠋⠀⠰⢸⣿⠀⠈⠄⠀⠀⠀⢻⣇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢀⣀⣠⣤⡾⠀⠈⠹⢷⣮⣛⡶⣎⣆⠏⣜⡩⢒⡱⢎⣔⣻⡿⠁⠀⠀⠹⣼⣿⡆⢁⠠⠀⠄⠀⠈⣏⣻⠀⠀⠀⠀⠀⠀
⢠⠴⠖⡲⠛⡭⣁⠐⣨⡿⠈⠀⠀⠀⠀⠉⠛⠛⠻⠻⠿⢶⣧⣷⣶⡷⡾⠋⠀⠀⠀⠀⠀⠉⢻⣷⡈⢠⠀⠀⠀⠀⢸⡟⠀⠀⠀⠀⠀⠀
⣀⠂⣏⠴⣉⠶⣈⢰⡿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⡀⢀⠀⠀⠰⣤⠀⠀⠀⠀⠈⠹⣧⡀⢄⠂⠀⠀⠀⣷⡆⠀⠀⠀⠀⠀
⠀⢀⢻⣞⡯⢷⣠⡿⠁⠀⠀⠀⠀⠀⣀⣼⣇⠀⠀⠀⠀⠀⠀⢘⡇⠀⠀⠀⠀⣿⣿⣦⣄⠀⠀⠀⢿⡔⡈⠔⡀⠀⠀⠸⣇⠀⠀⠀⠀⠀
⠀⠌⡎⢷⣹⢧⡿⠁⠀⠀⠀⠀⣠⡾⠋⠡⣿⠀⠀⠀⠀⠀⠀⣼⡆⠀⠀⠀⢠⣿⣿⡿⢿⠻⡶⣦⣼⣷⣁⠂⡠⠀⠀⠀⠛⠆⠤⢀⡀⠀
⠀⠘⡬⢻⣽⡿⠁⠀⠀⣀⣴⠟⠃⠄⡀⢆⣿⠀⠀⠀⠀⠀⣼⣿⣿⡀⠀⢀⣼⣿⣿⣟⣎⠳⡱⢈⠉⠹⣇⠂⠄⠑⠀⠀⠀⠀⠀⠀⠀⠀
⠀⢘⣰⣻⣿⠁⠀⣠⡾⢋⡅⣨⠐⠠⠐⢬⣿⠀⠀⠀⠀⣸⣟⡯⢿⣇⠀⣸⣿⣿⣚⣿⣎⡷⣑⠣⡄⠀⢫⠌⢂⠠⠀⠀⠀⠀⠀⠀⠀⠀

## Canva Presentation

[Canva link](https://www.canva.com/design/DAGdwyCR5m8/TQ8Al7K2aelqnAYpcZrqlA/view?utm_content=DAGdwyCR5m8&utm_campaign=designshare&utm_medium=link2&utm_source=uniquelinks&utlId=h2c34a26ee4)

## How does it work?

Kermit is a command-line tool that allows users to interact with a blockchain node and manage wallets through a set of subcommands. The CLI follows a structured format, where each command corresponds to a specific operation related to either blockchain node management or wallet operations.

Basic Structure:
The Kermit CLI is built using the Clap library, which organizes commands into a hierarchy of options and subcommands. The basic syntax for using Kermit is:

`kermit [COMMAND] [SUBCOMMAND] [OPTIONS]`

`COMMAND`: Represents the main category of operations (e.g., wallet, infos).

`SUBCOMMAND`: A specific action or operation under the selected command (e.g., create, status, list for wallet commands).

`OPTIONS`: Additional arguments required to complete the command (e.g., URL, wallet name, password).

All command is based on the Alephium API

## Getting Started

### Installation

To set up the CLI, follow this 2 steps:

1. Clone the Kermit CLI repository:

    ```bash
    git clone git@github.com:PoCInnovation/kermit.git
    ```

2. Install the program:

    ```bash
    cargo install --path .
    ```

#### !! If you want to run wallet command !!

#### Setup the Alephium Stack Development Environment

To set up the development environment, follow these steps:

1. Clone the Alephium Stack repository:

    ```bash
    git clone git@github.com:alephium/alephium-stack.git
    ```

2. Navigate to the `devnet` directory:

    ```bash
    cd alephium-stack/devnet
    ```

3. Start the development environment with Docker:

    ```bash
    docker-compose up -d
    ```


### Usage

You just need to run the command with the parameters of your choice

    ```bash
    kermit [COMMAND] [SUBCOMMAND] [OPTIONS]
    ```

### Examples

Here are some examples of how to use the Kermit CLI:
> Before using kermit, I recommand you to set the URL of Alephium API with the command `export URL=<URL>`

1. Create a new wallet:

    ```bash
    kermit wallet create <WALLET_NAME> <PASSWORD>
    ```

2. Transfer funds between wallets:

    ```bash
    kermit tx create <SENDER_PUBLIC_KEY> <RECIPIENT_ADDRESS> <AMOUNT> --private-key <SENDER_PRIVATE_KEY>
    ```

3. Check balance of a wallet:

    ```bash
    kermit address balance <ADDRESS>
    ```

4. Compile a contract:

    ```bash
    kermit contracts compile tests/contracts/sub_contract.ral --contract-type project
    ```

You can do again more with kermit. I let you check the `kermit --help`, to take a look on all possibilities

## Get involved

You're invited to join this project ! Check out the [contributing guide](./CONTRIBUTING.md).

If you're interested in how the project is organized at a higher level, please contact the current project manager.

## Our PoC team ❤️

Developers
| [<img src="https://github.com/alexandreTimal.png?size=85" width=85><br><sub>[Alexandre Timal]</sub>](https://github.com/alexandreTimal) | [<img src="https://github.com/Nfire2103.png?size=85" width=85><br><sub>[Nathan Flattin]</sub>](https://github.com/thomas-pommier-epi) | [<img src="https://github.com/thomas-pommier-epi.png?size=85" width=85><br><sub>[Thomas Pommier]</sub>](https://github.com/thomas-pommier-epi)
| :---: | :---: | :---: |

<h2 align=center>
Organization
</h2>

<p align='center'>
    <a href="https://www.linkedin.com/company/pocinnovation/mycompany/">
        <img src="https://img.shields.io/badge/LinkedIn-0077B5?style=for-the-badge&logo=linkedin&logoColor=white" alt="LinkedIn logo">
    </a>
    <a href="https://www.instagram.com/pocinnovation/">
        <img src="https://img.shields.io/badge/Instagram-E4405F?style=for-the-badge&logo=instagram&logoColor=white" alt="Instagram logo"
>
    </a>
    <a href="https://twitter.com/PoCInnovation">
        <img src="https://img.shields.io/badge/Twitter-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white" alt="Twitter logo">
    </a>
    <a href="https://discord.com/invite/Yqq2ADGDS7">
        <img src="https://img.shields.io/badge/Discord-7289DA?style=for-the-badge&logo=discord&logoColor=white" alt="Discord logo">
    </a>
</p>
<p align=center>
    <a href="https://www.poc-innovation.fr/">
        <img src="https://img.shields.io/badge/WebSite-1a2b6d?style=for-the-badge&logo=GitHub Sponsors&logoColor=white" alt="Website logo">
    </a>
</p>

> 🚀 Don't hesitate to follow us on our different networks, and put a star 🌟 on `PoC's` repositories

> Made with ❤️ by PoC
