#include <stdio.h>
#include <string.h>
#include <ctype.h>
#define BUFFER_SIZE 64

int getFirstVal(char string[]){
	//first search for first occurence of number char
	int earliestNumberIndex = -1;
	int earliestNumber = -1;
	for(int i = 0; i <= strlen(string); i++){
		int val = string[i] - '0';
		if(isdigit(string[i])){
			earliestNumberIndex = i;
			earliestNumber = val;
			break;
		}
	}

	//search string for spelling of each number. If occurs before earliest value, replace it
	char* one = strstr(string, "one");
	if(one != NULL){
		int position = one - string;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 1;
		}
	}
	

	char* two = strstr(string, "two");
	if(two != NULL){
		int position = two - string;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 2;
		}
	}

	char* three = strstr(string, "three");
	if(three != NULL){
		int position = three - string;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 3;
		}
	}
	
	char* four = strstr(string, "four");
	if(four != NULL){
		int position = four - string;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 4;
		}
	}

	char* five = strstr(string, "five");
	if(five != NULL){
		int position = five - string;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 5;
		}
	}

	char* six = strstr(string, "six");
	if(six != NULL){
		int position = six - string;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 6;
		}
	}

	char* seven = strstr(string, "seven");
	if(seven != NULL){
		int position = seven - string;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 7;
		}
	}

	char* eight = strstr(string, "eight");
	if(eight != NULL){
		int position = eight - string;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 8;
		}
	}

	char* nine = strstr(string, "nine");
	if(nine != NULL){
		int position = nine - string;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 9;
		}
	}

	return earliestNumber;
}

int getLastVal(char string[]){

	char rev[BUFFER_SIZE];

	for(int i = 0; i < strlen(string); i++){
		rev[i] = string[strlen(string) - 1 - i];
	}
	rev[strlen(string)] = '\0';

	//first search for first occurence of number char
	int earliestNumberIndex = -1;
	int earliestNumber = -1;
	for(int i = 0; i < strlen(rev); i++){
		int val = rev[i] - '0';
		if(isdigit(rev[i])){
			earliestNumberIndex = i;
			earliestNumber = val;
			break;
		}
	}

	//search string for spelling of each number. If occurs before earliest value, replace it
	char* one = strstr(rev, "eno");
	if(one != NULL){
		int position = one - rev;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 1;
		}
	}
	

	char* two = strstr(rev, "owt");
	if(two != NULL){
		int position = two - rev;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 2;
		}
	}

	char* three = strstr(rev, "eerht");
	if(three != NULL){
		int position = three - rev;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 3;
		}
	}
	
	char* four = strstr(rev, "ruof");
	if(four != NULL){
		int position = four - rev;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 4;
		}
	}

	char* five = strstr(rev, "evif");
	if(five != NULL){
		int position = five - rev;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 5;
		}
	}

	char* six = strstr(rev, "xis");
	if(six != NULL){
		int position = six - rev;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 6;
		}
	}

	char* seven = strstr(rev, "neves");
	if(seven != NULL){
		int position = seven - rev;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 7;
		}
	}

	char* eight = strstr(rev, "thgie");
	if(eight != NULL){
		int position = eight - rev;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 8;
		}
	}

	char* nine = strstr(rev, "enin");
	if(nine != NULL){
		int position = nine - rev;
		if(position < earliestNumberIndex){
			earliestNumberIndex = position;
			earliestNumber = 9;
		}
	}

	return earliestNumber;
}

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
			int firstDigit = getFirstVal(buffer);
			int secondDigit = getLastVal(buffer);

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

