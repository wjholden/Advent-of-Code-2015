using JSON3

function no_red_sum(obj)
    if isa(obj, JSON3.Object)
        if any("red" == v for v in values(obj))
            return 0
        end
    end

    total = 0
    for v in values(obj)
        if isa(v, Number)
            total += v
        elseif isa(v, String)
            continue
        else
            # I think that this case should be safe regardless
            # of if v is a JSON array, JSON object, or string.
            total += no_red_sum(v)
        end
    end
    total
end

print(no_red_sum(JSON3.read(ARGS[1])))
