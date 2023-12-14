points = 0

File.foreach("../input.txt") do |line|
    #split each line on the character '|'
    pair = line.split("|")
    
    #create winningNumbers string
    winningNumbersString = pair[0].slice(10...pair[0].length)
    #split winningNumbers on every third character
    winningNumbers = winningNumbersString.chars.each_slice(3).map(&:join).map(&:strip).map(&:to_i)
    
    ticketPoints = 0

    ticketNumbers = pair[1].slice(1...pair[1].length).chars.each_slice(3).map(&:join).map(&:strip).map(&:to_i)
    ticketNumbers.each do |number|
        if winningNumbers.include?(number)
            ticketPoints == 0 ? ticketPoints += 1 : ticketPoints += ticketPoints
        end
    end
    
    points += ticketPoints
end

puts points

