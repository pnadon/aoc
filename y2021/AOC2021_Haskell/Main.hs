import Day1
import Day2

main = do
  contents <- readFile "inputs/2.txt"
  --let nums = map read . words $ contents
  
  print $ Day2.soln1 $ map words $ lines contents
  --print $ Day1.soln2 nums

