#include <stdio.h>
#include <string.h>
#include <ctype.h>
#define BUFFER_SIZE 64
int main(int argc, char* argv[]){

	if(argc != 2){
		printf("Usage: ./a.out input.txt\n");
		return -1;
	}

	FILE* file;
	file = fopen(argv[1], "r");


	int total = 0;
	if(file != NULL){
		char buffer[BUFFER_SIZE];
		while(fgets(buffer, BUFFER_SIZE, file)){
			int firstDigit;
			for(int i = 0; i < BUFFER_SIZE; i++){
				int val = buffer[i] - '0';
				if(isdigit(buffer[i])){
					firstDigit = val; 
					break;
				}
			}
			int secondDigit;
			for(int i = strlen(buffer); i >= 0; i--){
				int val = buffer[i] - '0';
				if(isdigit(buffer[i])){
					secondDigit = val;
					break;
				}
			}

			total += (10 * firstDigit + secondDigit); 
		}
		fclose(file);
	} else {
		printf("File not found\n");
		return -1;
	}

	printf("%d\n", total);

	return 0;
}
