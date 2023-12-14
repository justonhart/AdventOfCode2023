import { Pull } from "./pull";

export class Game {
    gameNumber: number;
    pulls: Pull[];
    constructor(gameNumber: number, pulls: Pull[]){
        this.gameNumber = gameNumber;
        this.pulls = pulls;
    }
    power(): number {
        let minBlue: number = 0;
        let minRed: number = 0;
        let minGreen: number = 0;
        this.pulls.forEach(p => {
            if(p.blueCount > minBlue){
                minBlue = p.blueCount;
            }
            if(p.redCount > minRed){
                minRed = p.redCount;
            }
            if(p.greenCount > minGreen){
                minGreen = p.greenCount;
            }
        });

        return minBlue * minRed * minGreen;
    }
}