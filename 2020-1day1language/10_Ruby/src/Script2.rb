if ARGV.length != 1
  puts "Usage: Script1.rb <filename>"
  exit
end

# list of integers, each on a new line in the file
inputs = File.readlines(ARGV[0]).map(&:to_i)

def find_joltage(addapters, current_joltage, joltage_goal)
  # addapters can take a joltage 1-3 lower than their own
  available_joltages = addapters.select { |joltage| joltage - current_joltage <= 3 && joltage > current_joltage }
  output = []
  for joltage in available_joltages
    if joltage_goal - joltage <= 3
      output << [joltage]
    else
      result = find_joltage(addapters, joltage, joltage_goal)
      if result.length > 0
        result.each do |r|
          output << [joltage] + r
        end
      else
        return []
      end
    end
  end
  output
end

def get_diffs(joltages)
  diffs = {}
  last_joltage = 0
  for joltage in joltages
    diff = joltage - last_joltage
    diffs[diff] ||= []
    diffs[diff] << joltage
    last_joltage = joltage
  end
  diffs
end

inputs.sort!
puts inputs.inspect
result = find_joltage(inputs, 0, inputs.max + 3)
puts result.length

