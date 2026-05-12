# Iterator Thinking Checklist

- Is this code transforming data or controlling a process?
- Would an iterator chain make the transformation easier to read?
- Would a plain `for` loop be clearer here?
- Am I collecting too early?
- Am I allocating intermediate `Vec`s without a good reason?
- Does the chain express intent or just show off API knowledge?
- Are mapping, filtering, and formatting steps clearly separated?
- Can the reader see what the final shape of the data is?
- Did I choose an iterator because it improves the architecture, not because it looks advanced?
- Could the domain expose a better shape that makes iteration simpler?
