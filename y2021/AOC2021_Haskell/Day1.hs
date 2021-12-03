module Day1 where

import System.IO  
import Control.Monad
import Data.List

soln1 nums = length . filter tupleLT $ pairs nums
soln2 nums = length . filter tupleLT $ pairs $ map tupleSum $ triples nums

tupleLT :: (Int,Int) -> Bool
tupleLT (a,b) = a < b

tupleSum :: (Int,Int,Int) -> Int
tupleSum (a, b, c) = a + b + c

pairs :: [Int] -> [(Int,Int)]
pairs (x1:x2:xs) = [(x1, x2)] ++ (pairs (x2:xs))
pairs [x] = [(x,x)]
pairs [] = []

triples :: [Int] -> [(Int,Int,Int)]
triples (x1:x2:x3:xs) = [(x1, x2, x3)] ++ (triples (x2:x3:xs))
triples [x1,x2] = []
triples [x] = []