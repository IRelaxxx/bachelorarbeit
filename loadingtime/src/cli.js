#!/usr/bin/env node

function filterEntries(entries, whitelist) {
    let filteredentries = entries.filter(e => whitelist.map(w => e.request.url.match(w) !== null).some(e => e === true));
    let minStartDates = filteredentries.map(e => e.startedDateTime).sort();
    let maxEndDates = filteredentries.map(e => {e.startedDateTime; e.time}).map(e => addMs(new Date(e.startedDateTime), e.time)).sort();
    return {entries: filteredentries, time: maxEndDates[maxEndDates.length - 1].getTime() - minStartDates[0].getTime()};
}

function addMs(date, ms) {
    return new Date(date.getTime() + ms)
}

const { program } = require('commander');
const fs = require('fs');

//const program = new Command();
program.version('1.0.0');

program.option('-o, --output <filepath>', "Output filepath");
program.requiredOption('-w, --whitelist <filepath>', "Whilelist file with one URL per line. UTF-8 encoded.")
program.requiredOption('-i, --input <filepath>', "Input har file path");
program.parse(process.argv)

let inputFilePath = program.input.trim();
let whitelistFilePath = program.whitelist.trim();
let outputFilePath = program.output.trim();

if(!fs.existsSync(inputFilePath)) {
    console.log("input file does not exist");
    return
}

if(!fs.existsSync(whitelistFilePath)) {
    console.log("Whitelist file does not exist");
    return
}

let input = JSON.parse(fs.readFileSync(inputFilePath, {encoding: "utf8"}));
let whitelist = fs.readFileSync(whitelistFilePath, {encoding: "utf8"}).split(/\r\n|\r|\n/);

let filterOutput = filterEntries(input.log.entries, whitelist);

input.time = filterOutput.time;
input.log.entries = filterOutput.entries;

fs.writeFileSync(outputFilePath, input, {encoding: "utf8"});


