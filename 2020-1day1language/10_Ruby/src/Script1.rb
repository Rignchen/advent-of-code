if ARGV.length != 1
  puts "Usage: Script1.rb <filename>"
  exit
end

# list of integers, each on a new line in the file
inputs = File.readlines(ARGV[0]).map(&:to_i)

