# tempo-bot

For those who are tired to log their time on Tempo every day, and want to focus on their work, witness the **Tempo Bot 🤖**.

It's a CLI to log your time on Tempo automatically, based on your current tickets in Jira.

Run it every day using a cron-job or a cloud-function, and don't worry anymore about reporting !

> **Built with Rust.** Why ? Well, because I'm experimenting with Rust on my free-time (so the code may not be very idiomatic 🙃)

## Usage

```
Log your time on Tempo automatically, based on your current tickets in Jira.

USAGE:
    tempo-bot [OPTIONS] --jira-host <JIRA_HOST> --board-id <BOARD_ID> --sprint-prefix <SPRINT_PREFIX> --date <DATE> --email <EMAIL> --api-key <API_KEY> --tempo-api-key <TEMPO_API_KEY> --account-id <ACCOUNT_ID>

OPTIONS:
    -a, --account-id <ACCOUNT_ID>
            User's account id (used to login in Tempo). Can be found in the url of your profile page

        --api-key <API_KEY>
            Jira API key. Can be generated from https://id.atlassian.com/manage/api-tokens

    -b, --board-id <BOARD_ID>
            The Board ID where your sprints live. Can be found in the url of your backlog/sprint
            page

    -d, --date <DATE>
            The date to log time for. Accepted formats: 'today', 'yesterday', 'YYYY-MM-DD'

        --day-duration <DAY_DURATION>
            Target working day duration (in hours). If not specified, the default day duration from
            Tempo is used

        --dry-run
            Dry run mode. If specified, no time will be logged

    -e, --email <EMAIL>
            Email (used to login in Jira)

    -h, --help
            Print help information

    -j, --jira-host <JIRA_HOST>
            The Jira host name. If you are using Jira Cloud, it's likely '<company>.atlassian.net'

    -s, --sprint-prefix <SPRINT_PREFIX>
            The prefix used for your sprints, without the '#'

        --tempo-api-key <TEMPO_API_KEY>
            Tempo API key. Can be generated from "Tempo > Settings > API integration" in your
            dashboard

    -V, --version
            Print version information

        --work-increment <WORK_INCREMENT>
            Increment of a work log (in minutes). Every work lok will be rounded to a multiple of
            this increment [default: 30]

        --work-min-duration <WORK_MIN_DURATION>
            Minimal duration of a work (in minutes). Any work below this threshold will be skipped
            [default: 15]

    -y, --yes
            Answer 'yes' to all question, thus bypassing any user input
```

- **Cronjob:** run it every day at 19:00
```
0 19 * * * /path/to/tempo-bot --jira-host ...
```

## CLI Workflow

The CLI works as follows:
1. Fetch all the necessary info from Tempo and Jira (sprints, issues, already logged time)
2. Assign to each issue a "time score" (based on the story points, the status, the assignee, the priority, and a bit of randomness !)
3. Using these time scores, assign a time for each issue
4. Send these "worklogs" to the Tempo API
5. Done !

## References API documentation

This CLI uses these 2 APIs:

- [Jira Cloud Rest API](https://developer.atlassian.com/cloud/jira/software/rest/intro/)
- [Tempo Cloud Rest API](https://apidocs.tempo.io/)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
