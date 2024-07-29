using FluentAssertions;

namespace Template.Tests.Appium;

public class UnitTest1
{
    [Fact]
    public void Test1()
    {
        (1 + 1).Should().Be(2);
    }
}