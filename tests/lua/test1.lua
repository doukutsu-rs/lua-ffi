-- Point2D is a struct defined in rust
point = Point2D:new()
point:setX(2)
point:setY(4)

if point:add() ~= 6 then
    error()
end