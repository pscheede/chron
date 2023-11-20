# chron
CLI tool for tracking time

![build workflow](https://github.com/pscheede/chron/actions/workflows/build_test.yml/badge.svg?event=push)
![lint workflow](https://github.com/pscheede/chron/actions/workflows/lint.yml/badge.svg?event=push)

## Vision

Build a tool for quick and easy time tracking at work.

## Roadmap

### Tracking

Tracking will work in chunks.
Each chunk will be assigned to one project.
Break chunks will also exist.

In the morning the check-in time needs to be tracked.
Then, at the end of each chunk, the work from the previous chunk needs to be tracked,
i.e. assigned to a project.

There will be no separate mark for the end of the day.

Days should be able to be marked as days were there was no work, example for illness or vacation, so that you know in retrospect why there is no time tracked for that day.

#### Chunk creation

- Checking in in the morning: `chron check-in`
- Tracking a new project-chunk: `chron track <project> <description>`
- Tracking a break chunk: `chron break <description>`

Descriptions will be optional.

In case a chunk was forgotten, there should be a possibility to add it at a later point.
For this, you would only need to insert a new endpoint of a chunk.
Editing a day in the past is not planned as of now, in order to train the user to track
their times reliably.

`chron retrotrack <timestamp> <project> <description>`

#### Project management

Projects must be created beforehand, they cannot be created dynamically.
This way, typos don't lead to the creation of new projects.

- Create new project: `chron projects new <name> <description>`
- Delete project: `chron projects delete <name> <description>`
- List projects: `chron projects list`

Deleted projects must still occur in the final analysis, but they cannot be used
for tracking anymore.

Optional feature: Aliases for projects, allow easier tracking, with less congnitive load.
A daily meeting, for example, can always be added to the same project.
`chron track daily` would then automatically select the right project.

### Log

Tracking would not be useful, if the tracked times cannot be logged.

Logging must be possible per day, per week, and maybe even per month.

The log must contain an acumulation of times per project per day.
It must also contain the total amount of work per day.

It would also be nice to have an acumulation per week/month for each project.

The exact output format is not yet clear, but it should be easily adaptable.

- `chron log today`
- `chron log yesterday`
- `chron log <date>`
- `chron log week <number>`
- `chron log month <name>`

