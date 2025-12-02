module Main where

import System.Environment (getArgs)

-- ==================================================================
-- Setup
-- ==================================================================

data Instruction = L Int | R Int
type Dial = (Int, Int)
--           |    |
--           |    Zero Count
--           Position

parseLine :: String -> Instruction
parseLine [] = error "Empty input"
parseLine (dir:num)
  | dir == 'L' = L (read num)
  | dir == 'R' = R (read num)
  | otherwise = error "Invalid input"

parseInput :: String -> [Instruction]
parseInput = map parseLine . lines

-- ==================================================================
-- Part 1
-- ==================================================================

finalPosition :: Int -> Instruction -> Int
finalPosition position (L value) = mod (position - value) 100
finalPosition position (R value) = mod (position + value) 100

onZero :: Int -> Int
onZero n | n == 0 = 1
         | otherwise = 0

rotatePart1 :: Dial -> Instruction -> Dial

rotatePart1 (position, count) instruction = (position', count')
  where position' = finalPosition position instruction
        isZero = onZero position'
        count' = count + isZero

solvePart1 :: [Instruction] -> Int
solvePart1 instructions = snd $ foldl rotatePart1 (50, 0) instructions

-- ==================================================================
-- Part 2
-- ==================================================================

getValue :: Instruction -> Int
getValue (L value) = value
getValue (R value) = value

netMovement :: Instruction -> Int
netMovement = flip mod 100 . getValue

cycleCount :: Instruction -> Int
cycleCount = flip div 100 . getValue

distanceToZero :: Int -> Instruction -> Int
distanceToZero position (L _) = position
distanceToZero position (R _) = 100 - position

onCross :: Int -> Instruction -> Int
onCross position instruction
  | position == 0 = 0
  | movement > distanceToZero position instruction = 1
  | otherwise = 0
  where movement = netMovement instruction

rotatePart2 :: Dial -> Instruction -> Dial
rotatePart2 (position, count) instruction = (position', count')
  where cycles = cycleCount instruction
        didCross = onCross position instruction
        position' = finalPosition position instruction
        isZero = onZero position'
        count' = count + cycles + didCross + isZero

solvePart2 :: [Instruction] -> Int
solvePart2 instructions = snd $ foldl rotatePart2 (50, 0) instructions

-- ==================================================================
-- Main
-- ==================================================================

main :: IO ()
main = do
  args <- getArgs
  input <- readFile "input/input.txt"
  let instructions = parseInput input
  case args of
    ["1"] -> print . solvePart1 $ instructions
    ["2"] -> print . solvePart2 $ instructions
    _     -> putStrLn "Usage: ./Main [1|2]"

