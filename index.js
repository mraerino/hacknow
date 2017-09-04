#!/usr/bin/env node

const fs = require('mz/fs');
const os = require('os');
const path = require('path');
const url = require('url');
const parseArgs = require('command-line-args');
const git = require('git-promise');
const emoji = require('node-emoji');

const exitClean = (code = 1) => { console.log('.'); process.exit(code); };

const args = parseArgs([
    { name: 'repo', alias: 'r', defaultOption: true, type: String },
    { name: 'project-dir', alias: 'd', type: String },
    { name: 'ssh', type: Boolean }
]);

if(!('repo' in args)) {
    console.error(`${emoji.get('warning')}  No repo specified!`);
    exitClean();
}
if(args.repo.split('/').length !== 2) {
    console.error(`${emoji.get('warning')}  Repo needs to contain a slash!`);
    exitClean();
}

const repo = args.repo;
const dir = ('project-dir' in args) ? args['project-dir'] : os.homedir();
const fullpath = path.join(dir, repo);
const gitRemote = (('ssh' in args) && args.ssh === true)
    ? `git@github.com:${repo}`
    : url.resolve("https://github.com/", repo);

fs.stat(fullpath)
    .then(stat => stat.isDirectory()
        ? Promise.resolve(true)
        : console.error(`${emoji.get('warning')}  On the desired path is a file, therefore we can not create a directory there`) || exitClean()
    )
    .catch(err => Promise.resolve(false))
    .then(exists => exists
        ? Promise.resolve()
            .then(() => console.error(`${emoji.get('arrow_down')}  Fetching from origin...`))
            .then(() => git("fetch --all", { cwd: fullpath }))
            .then(() => console.error(`${emoji.get('white_check_mark')}  Repository overview:`))
            .then(() => git("status", { cwd: fullpath }))
            .then(status => console.error(status))
        : Promise.resolve()
            .then(() => console.error(`${emoji.get('arrow_down')}  Cloning into new directory...`))
            .then(() => git(`clone ${gitRemote} ${fullpath}`))
            .then(() => console.error(`${emoji.get('white_check_mark')}  Repository ready`))
    )
    .then(() => { console.log(fullpath); process.exit(0); })
    .catch(err => console.error(`${emoji.get('warning')}  Git command failed:\n${err.stdout}`))
    .then(() => exitClean());