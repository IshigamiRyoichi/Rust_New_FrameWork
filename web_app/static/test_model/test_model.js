window.addEventListener('DOMContentLoaded', function() {
    // fitCanvasSize();
    // window.onresize = fitCanvasSize;
    createPlace();
});

var arrayPlace = [];
var arrayArc = [];

function fitCanvasSize() {
    // canvasの種類を取得
    var canvas_model = document.getElementById("model-canvas");
    // Canvas のサイズをクライアントサイズに合わせる
    model_canvas.width = document.documentElement.clientWidth - 50;
    model_canvas.height = 100;
    canvas_model.width = document.documentElement.clientWidth - 50;
    canvas_model.height = 1000;
}

async function createPlace(){
    var node_num;
    var arc_num;
    var text_name = 'http://127.0.0.1:5000/../../test_model/data_txt/controller_name.txt';
    var json_name = 'http://127.0.0.1:5000/../../test_model/data_json/model_file.json';
    var parts_data = await fetch(json_name);
    if (parts_data.ok){
        var json = await parts_data.json();
        console.log("success!!")
        console.log(json);
        node_num = json.parts.controller;
        arc_num = json.parts.form;
    }
    var controller_data = await fetch(text_name);
    var file_text = await controller_data.text();
    var controller_array = file_text.split(/\r\n|\n/);
    // console.log(file_text);
    // for (var line = 0; line < controller_array.length-1; line++){
    //     console.log("text");
    //     console.log(controller_array[line]);
    // }
    var model_canvas = new fabric.Canvas("model-canvas");

    //フリー描画禁止
    model_canvas.isDrawingMode = false;

    for(var i = 0; i < node_num; i++){
        var r = 60;
        var x_place = ((i + 1) * model_canvas.width - r / 2) / (1 + node_num);
        var y_place = model_canvas.height / 5;
        // var start_ang = 0 * Math.PI / 180;
        // var finish_ang = 360 * Math.PI / 180;
        var object_name = "place";

        var place_obj = new fabric.Circle({
            id: object_name + 1,
            originX: "center",
            originY: "center",
            left: x_place,
            top: y_place,
            radius: r,
            stroke: 'black',
            fill: "rgb(240,240,240)",
        });
        // model_canvas.add(place_obj);
        var controller_name = controller_array[i];
        var place_text = new fabric.Text(controller_name, {
            originX: "center",
            originY: "center",
            left: x_place,
            top: y_place,
            fill: 'red',
            stroke: 'black',
            strokeWidth: 2,
            fontFamily: 'Ariel',
            fontSize: 20
        });
        // model_canvas.add(place_text);
        var place_all = new fabric.Group([place_obj, place_text], {
            originX: "center",
            originY: "center",
            left: x_place,
            top: y_place
        });
        model_canvas.add(place_all)
        arrayPlace.push(place_obj);
    }
    for(var i = 0; i < arc_num; i++){
        // 開始点
        var x_arc_1 = (i + 0.97) * model_canvas.width / (1 + arc_num);
        var y_arc_1 = 3 * model_canvas.height / 4;
        // 中間点
        var x_arc_2 = x_arc_1 + 60
        var y_arc_2 = y_arc_1
        var x_arc_3 = x_arc_2 + 10 * Math.cos(5 * Math.PI / 6);
        var y_arc_3 = y_arc_2 + 10 * Math.sin(5 * Math.PI / 6);
        var x_arc_4 = x_arc_2 + 10 * Math.cos(7 * Math.PI / 6);
        var y_arc_4 = y_arc_2 + 10 * Math.sin(7 * Math.PI / 6);
        var arc_obj = new fabric.Polyline([
            {x: x_arc_1, y: y_arc_1},       //始点
            {x: x_arc_2 , y: y_arc_2},      //中間点1
            {x: x_arc_3 , y: y_arc_3},      //中間点2
            {x: x_arc_2 , y: y_arc_2},      //中間点3
            {x: x_arc_4 , y: y_arc_4},      //中間点3
            {x: x_arc_2 , y: y_arc_2}],{    //終点
            strokeWidth: 3,       //線の太さ
            stroke: 'black',
        });
        model_canvas.add(arc_obj);
        arrayArc.push(arc_obj);
    }

}