using Microsoft.AspNetCore.Mvc;
using MyApi.Controllers;
var builder = WebApplication.CreateBuilder(args);

// Добавляем поддержку контроллеров
builder.Services.AddControllers();

var app = builder.Build();

// Маршрутизация для контроллеров
app.MapControllers();

// Минимальный API endpoint
app.MapGet("/", () => "Hello World!");

app.Run();