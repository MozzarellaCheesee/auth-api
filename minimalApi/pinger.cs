using Microsoft.AspNetCore.Mvc;

namespace MyApi.Controllers;

[ApiController]
[Route("api/[controller]")]
public class PingerController : ControllerBase
{
    [HttpGet("ping")]
    public IActionResult Get()
    {
        var response = new
        {
            Message = "Понг! Ахуеть, да?",
        };

        return Ok(response);
    }
}