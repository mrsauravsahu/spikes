using Codeuctivity.ImageSharpCompare;
using OpenQA.Selenium.Appium;
using OpenQA.Selenium.Appium.Enums;
using OpenQA.Selenium.Appium.iOS;
using OpenQA.Selenium.Remote;
using SixLabors.ImageSharp;
using FluentAssertions;
using OpenQA.Selenium;
using OpenQA.Selenium.Support.UI;

namespace Template.Tests.Appium;

public class ScreenshotTests : IDisposable
{
    private readonly IOSDriver<IOSElement> _driver;

    public ScreenshotTests()
    {
        var appId = "in.mrsauravsahu.countryquiz";


        var driverOptions = new AppiumOptions();
        driverOptions.AddAdditionalCapability(MobileCapabilityType.PlatformName, "iOS");
        driverOptions.AddAdditionalCapability("appium:automationName", "XCUITest");
        driverOptions.AddAdditionalCapability(MobileCapabilityType.App, appId);
        driverOptions.AddAdditionalCapability(MobileCapabilityType.DeviceName, "iPhone Simulator");
        driverOptions.AddAdditionalCapability(MobileCapabilityType.PlatformVersion, "17.5");

        _driver = new IOSDriver<IOSElement>(new Uri("http://localhost:4723"), driverOptions);
        _driver.ActivateApp(appId);
    }

    public void Dispose() => _driver.Quit();

    [Theory]
    [InlineData("AddTodo")]
    public void Should_Match_Screenshot(string scenario)
    {
        var screenshotsPathPrefix = "/Users/Saurav_Sahu/Downloads/mrsauravsahu/code/spikes/svc/appium-testing-ios/screenshots";
        var expectedPath = $@"{screenshotsPathPrefix}/figma-designs/{scenario}--iOS Large.png";
        var actualPath = $@"{screenshotsPathPrefix}/{scenario}.png";
        var diffPath = $@"{screenshotsPathPrefix}/diff/{scenario}.png";

        using (var fileStreamDifferenceMask = File.Create(diffPath))
        using (var maskImage = ImageSharpCompare.CalcDiffMaskImage(actualPath, expectedPath))
        { ImageExtensions.SaveAsPng(maskImage, fileStreamDifferenceMask); }

        var maskedDiff = ImageSharpCompare.CalcDiff(actualPath, expectedPath, diffPath);

        maskedDiff.AbsoluteError.Should().Be(0);
    }

    // [Fact]
    public void Should_AddTodo()
    {
        var todoText = "Buy groceries";

        var todoField = _driver.FindElement(By.XPath("//XCUIElementTypeTextField[@value=\"Type todo to complete...\"]"));

        todoField.Click();
        todoField.SendKeys(todoText);

        // Find the button with the text "Add todo" and click it
        var addButton = _driver.FindElement(By.XPath("//XCUIElementTypeButton[@name=\"Add todo\"]"));
        addButton.Click();

        var todo = _driver.FindElement(By.XPath($"//XCUIElementTypeButton[@name=\"{todoText}\"]"));

        Assert.NotNull(todo);

        var scenario = "AddTodo";
        var screenshotsPathPrefix = "/Users/Saurav_Sahu/Downloads/mrsauravsahu/code/spikes/svc/appium-testing-ios/screenshots";
        var actualPath = $@"{screenshotsPathPrefix}/{scenario}.png";

        var photo = _driver.GetScreenshot();
        photo.SaveAsFile(actualPath);
    }
}
