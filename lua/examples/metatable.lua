

Config = {}
Config.__call = function(t, props)
    -- print("call to table ".. tostring(t) .. " with arguments: ".. ...)
    -- print("All elements of the table:")
    for k,v in pairs(props) do
      t[k] = v
    end
end

java = { 
  peo = 1,
  foo = "no",
  directories = {
    "src/main/java"
  }
}

setmetatable(java, Config)
setmetatable(java.directories, Config)

java {
  source = 1.5,
  target = 1.6,
  caca = (function() return 2*2 end)(),
  
}

--java.directories {
--    "src/main/java2"
--  }

for k,v in pairs(java) do
  if type(v) == "table" then
    for k2,v2 in pairs(v) do print(k, "--> ", k2, tostring(v2)) end
  else
    print(k, tostring(v))
  end
end