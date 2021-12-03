module Day2 where
import Data.List.NonEmpty
import Data.Semigroup

data Direction = Forward | Up | Down deriving (Eq,Enum,Show)  

data Move = Move { dir :: Direction
                 , units :: Int } deriving (Eq)

data PositionOffset = PositionOffset { vert :: Int
              , hori :: Int } deriving Show

instance Semigroup PositionOffset where
  PositionOffset{vert=a, hori=b} <> PositionOffset{vert=c, hori=d} = PositionOffset{vert=(a + c), hori=(b + d)}

moveToPos :: Move -> PositionOffset
moveToPos Move{dir=Forward, units=u}  = PositionOffset{vert=0, hori=u}
moveToPos Move{dir=Up, units=u }       = PositionOffset{ vert=u, hori=0}
moveToPos Move{dir=Down, units=u }     = PositionOffset{ vert=(-u), hori=0}

rawToDirection "forward" = Forward
rawToDirection "up" = Up
rawToDirection "down" = Down
rawToDirection _ = error "invalid direction"

rawToMove :: [String] -> Move
rawToMove (d:u:xs) = Move{dir=(rawToDirection d), units=(read u)}
rawToMove _ = error "should be 2 or more words"

rawToPos :: [String] -> PositionOffset
rawToPos input = moveToPos $ rawToMove input

soln1 :: [[String]] -> PositionOffset
soln1 input = sconcat $ PositionOffset{vert=0,hori=0} :| Prelude.map rawToPos input

