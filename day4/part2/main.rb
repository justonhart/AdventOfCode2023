totalCards = 0

bonusCards = {}

File.foreach("../input.txt") do |line|
    #split each line on the character '|'
    pair = line.split("|")
    
    gameNo = pair[0].slice(5..7).to_i;
    
    #create winningNumbers string
    winningNumbersString = pair[0].slice(10...pair[0].length)
    #split winningNumbers on every third character
    winningNumbers = winningNumbersString.chars.each_slice(3).map(&:join).map(&:strip).map(&:to_i)
    
    cardsWon = 0

    ticketNumbers = pair[1].slice(1...pair[1].length).chars.each_slice(3).map(&:join).map(&:strip).map(&:to_i)
    ticketNumbers.each do |number|
        if winningNumbers.include?(number)
            cardsWon += 1
        end
    end 

    puts "Game #{gameNo} ( * #{1 + (bonusCards[gameNo] || 0).to_i}) - #{cardsWon} cards won"
    
    if cardsWon > 0
        for i in 1..cardsWon
            if bonusCards[gameNo + i] == nil
                bonusCards[gameNo + i] = 1 + (bonusCards[gameNo] || 0).to_i
            else
                bonusCards[gameNo + i] += 1 + (bonusCards[gameNo] || 0).to_i
            end
        end
    end

    totalCards += 1 + (bonusCards[gameNo] || 0).to_i
end

puts totalCards