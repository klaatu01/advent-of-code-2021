open System.IO;

let part1 (lines: string list) =
    lines
    |> List.map (fun x -> x.Split(' ') |> (fun y -> (y.[0], y.[1] |> int)))
    |> List.fold 
        (fun (a:int,b:int) x -> 
           match x with
           | ("forward", n) -> (a+n, b)
           | ("up", n) -> (a, b-n)
           | ("down", n) -> (a, b+n)
           | _ -> (a,b) )
        (0, 0)
    |> fun (a,b) -> a * b

let part2 (lines: string list) =
    lines
    |> List.map (fun x -> x.Split(' ') |> (fun y -> (y.[0], y.[1] |> int)))
    |> List.fold 
        (fun (depth:int, horizonal:int,aim:int) x -> 
           match x with
           | ("forward", n) -> (depth + (n * aim), horizonal + n, aim)
           | ("up", n) -> (depth, horizonal, aim - n)
           | ("down", n) -> (depth, horizonal, aim + n)
           | _ -> (aim, horizonal, depth) )
        (0, 0, 0)
    |> fun (a,b,_) -> a * b

    
    
[<EntryPoint>]
let main _ =
    let lines = File.ReadLines "./input.txt"
                |> List.ofSeq

    let p1 = part1 lines
    printfn "%i" p1

    let p2 = part2 lines
    printfn "%i" p2
    0
