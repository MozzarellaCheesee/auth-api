using Microsoft.AspNetCore.Mvc;
using MyApi.Controllers;
var builder = WebApplication.CreateBuilder(args);

// ��������� ��������� ������������
builder.Services.AddControllers();

var app = builder.Build();

// ������������� ��� ������������
app.MapControllers();

// ����������� API endpoint
app.MapGet("/", () => "Hello World!");

app.Run();