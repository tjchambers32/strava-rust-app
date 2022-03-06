# strava-spa
This is a simple application to display my yearly running metrics.

Strava's UI doesn't provide a clear chart of your mileage throughout the year.
"How many miles had I run this year on June 1st?" is a question that currently can't be answered using Strava out of the box.

To do this, we use Strava's API to query an [athlete's activities] (https://developers.strava.com/docs/reference/#api-Activities-getLoggedInAthleteActivities), we then parse this and update a CSV file on disk (each athlete has their own CSV file).