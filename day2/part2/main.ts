import { Game } from "./game";
import * as fs from 'fs';
import * as readline from 'readline';
import { Pull } from "./pull";

const RED_COUNT = 12;
const BLUE_COUNT = 14;
const GREEN_COUNT = 13;

const GAMES: Game[] = [];
const readable = fs.createReadStream('input.txt');
const rl = readline.createInterface({
    input: readable
});
rl.on('line', line => {
    let gameNo: number;
    const pulls: Pull[] = [];

    const firstSplit = line.split(":");   
    gameNo = Number(firstSplit[0].replace("Game ","").replace(":",""));
    firstSplit[1].split(";").forEach(pull => {
        const pullColors = pull.split(",").map(c => c.trim()).map(c => { return {count: Number(c.split(" ")[0]), color: c.split(" ")[1]}});
        const blue = pullColors.find(c => c.color === "blue")?.count ?? 0;
        const green = pullColors.find(c => c.color === "green")?.count ?? 0;
        const red = pullColors.find(c => c.color === "red")?.count ?? 0;
        
        pulls.push({redCount: red, greenCount: green, blueCount: blue});
    });

    GAMES.push(new Game(gameNo, pulls));
});
rl.on("close", () => {
    console.log(GAMES.reduce((total, nextGame) => total + nextGame.power(), 0));
});
