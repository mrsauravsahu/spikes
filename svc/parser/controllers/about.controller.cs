using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace Monorepo.Parser.Controllers
{
    [ApiController]
    public class AboutController : ControllerBase
    {
        private readonly ILogger<AboutController> _logger;

        public AboutController(ILogger<AboutController> logger)
        {
            _logger = logger;
        }

        [HttpGet("/")]
        public dynamic Get() => new
        {
            Name = "Parser Service"
        };
    }
}
