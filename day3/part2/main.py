symbolPositionsByLine = {}
numbersByLine = {}

with open('../input.txt', 'r') as f:
    lines = f.readlines()
    f.close()
    
    numberBuffer = None
    numberStart = None
    numberLen = None

    for lineidx, line in enumerate(lines):
        for charidx, char in enumerate(line.strip()):
            if(char.isnumeric()):
                if numberBuffer is None:
                    numberBuffer = char
                    numberStart = charidx
                    numberLen = 1
                else:
                    numberBuffer += char
                    numberLen += 1
            else:
                if numberBuffer is not None:
                    if lineidx not in numbersByLine:
                        numbersByLine[lineidx] = []
                    numbersByLine[lineidx].append((numberStart, numberLen, numberBuffer))
                    numberBuffer = None
                    numberLen = None
                    numberStart = None
                if(char != '.' and not(char.isnumeric())):
                    if lineidx not in symbolPositionsByLine:
                        symbolPositionsByLine[lineidx] = []
                    symbolPositionsByLine[lineidx].append((charidx, char))

        if numberBuffer is not None:
            if lineidx not in numbersByLine:
                numbersByLine[lineidx] = []
            numbersByLine[lineidx].append((numberStart, numberLen, numberBuffer))
            numberBuffer = None
            numberLen = None
            numberStart = None
 
gearRatioSum = 0

#for each * symbol with exactly two adjacent numbers, add the ratio of the two numbers to gearRatioSum
for line in symbolPositionsByLine:
    for symbol in symbolPositionsByLine[line]:
        if symbol[1] == '*':
            numbers = []
            #get all numbers in the line at positions between symbol[1] - 1 and symbol[1] + 1
            if (line - 1) in numbersByLine.keys():
                for numberTuple in numbersByLine[line - 1]:
                    if numberTuple[0] + numberTuple[1] - 1 >= symbol[0] - 1 and numberTuple[0] <= symbol[0] + 1:
                        numbers.append(int(numberTuple[2]))

            if (line + 1) in numbersByLine.keys():
                for numberTuple in numbersByLine[line + 1]:
                    if numberTuple[0] + numberTuple[1] - 1 >= symbol[0] - 1 and numberTuple[0] <= symbol[0] + 1:
                        numbers.append(int(numberTuple[2]))
                        
            if line in numbersByLine.keys():
                for numberTuple in numbersByLine[line]:
                    if numberTuple[0] + numberTuple[1] - 1 == symbol[0] - 1 or numberTuple[0] == symbol[0] + 1:
                        numbers.append(int(numberTuple[2]))

            if len(numbers) == 2:
                gearRatioSum += numbers[0] * numbers[1]
            
            print( line, symbol, numbers)

print(gearRatioSum)