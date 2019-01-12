module Main (main) where

import           Options.Applicative
import           System.Exit          (exitFailure)
import           System.IO            (hPutStrLn, stderr)
import           Keel.Main            (runKeel)

data CmdArgument = CmdArgument
  { inputFile   :: Maybe String
  , showVersion :: Bool
  , showHelp    :: Bool
  }

options :: IO CmdArgument
options = customExecParser pref information
  where
    information = info (helper <*> opts)
      $  fullDesc
      <> header "The Compiler for Keel Programming Language"
    pref = prefs $  showHelpOnError
                 <> showHelpOnEmpty
                 <> disambiguate
                 <> columns 80
    opts = CmdArgument
      <$> optional
        (strOption $  long "src"
                   <> help "Source file path"
                   <> metavar "PATH"
                   <> short 'c'
                   )
      <*> switch
          (  long "version"
          <> help "Show Keel Compiler version"
          <> short 'V'
          )
      <*> switch
          (  long "help"
          <> help "Show help message"
          <> short 'h'
          )

noInputFile :: CmdArgument -> IO a
noInputFile opts = do
  if not (showVersion opts || showHelp opts) then
    hPutStrLn stderr "Please specify an input file."
  else
    pure ()
  exitFailure

main :: IO ()
main = do
  args <- options
  file <- maybe (noInputFile args) pure $ inputFile args
  putStrLn file
  runKeel
