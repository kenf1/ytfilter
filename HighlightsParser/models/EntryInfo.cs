public class EntryInfo
{
    public string Id { get; set; } = "N/A";
    public string Published { get; set; } = "N/A";
    public string Updated { get; set; } = "N/A";
    public MediaGroupInfo MediaGroup { get; set; } = new MediaGroupInfo();
}

public class MediaGroupInfo
{
    public string MediaTitle { get; set; } = "N/A";
    public string MediaContentUrl { get; set; } = "N/A";
    public string MediaDescription { get; set; } = "N/A";
    public MediaCommunityInfo Community { get; set; } = new MediaCommunityInfo();
}

public class MediaCommunityInfo
{
    public StarRatingInfo StarRating { get; set; } = new StarRatingInfo();
    public StatisticsInfo Statistics { get; set; } = new StatisticsInfo();
}

public class StarRatingInfo
{
    public string Count { get; set; } = "N/A";
    public string Average { get; set; } = "N/A";
    public string Min { get; set; } = "N/A";
    public string Max { get; set; } = "N/A";
}

public class StatisticsInfo
{
    public string Views { get; set; } = "N/A";
}
