const readline = require('readline');
const fs = require('fs');

const readable = fs.createReadStream('../input.txt');
const reader = readline.createInterface({ input: readable });

let times = [];
let distances = [];

let firstLine = true;
reader.on('line', (line) => {
    let tokens = line.replace(/\s+/g, ' ').split(' ');
    if(firstLine) {
        times.push(...tokens.slice(1, tokens.length).map((x) => parseInt(x)));    
    } else {
        distances.push(...tokens.slice(1, tokens.length).map((x) => parseInt(x)));
    }
    firstLine = false;
});

reader.on('close', () => {
    let winningCombinationsByRace = [];
    //for each race
    for(let raceNumber = 0; raceNumber < times.length; raceNumber++) {
        let targetDistance = distances[raceNumber];
        //calculate the number of possible charge times that beat the target distance
        
        let winningTimes = [];
        for(let chargeTime = 0; chargeTime < times[raceNumber]; chargeTime++) {
            if((times[raceNumber] - chargeTime) * chargeTime > targetDistance) {
                winningTimes.push(chargeTime);
            } 
        }
        
        winningCombinationsByRace[raceNumber] = winningTimes.length;
    }

    console.log(winningCombinationsByRace.reduce((a, b) => a * b, 1));
});