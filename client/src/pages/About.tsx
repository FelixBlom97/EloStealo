export const About = () => {
    return (<div className="flex flex-col items-center justify-center font-bold"> 
        <h1 className="text-5xl p-8">Elo stealo chess</h1>
        <div className="max-w-5xl text-lg">Do you want to play chess with your friend, but is the Elo difference
        so big that the games are just kinda sad? Are you bored of regular chess and looking for new ways to play the game?
        Elo Stealo Chess is here to level the playing field and offer exciting new ways to play! <br />
        In Elo Stealo chess, one or both players receive a handicap, called an Elo Stealo rule. Each rule has a guesstimate of how much
        Elo one looses for having to play with this rule. <br /> Players should not know each other's Elo Stealo rules! <br/>
        If you can't move due to your Elo Stealo rule, you lose. Stillmate is still a draw.<br />
        You can either select a stealo rule yourself (keep it secret from your opponent!), or fill in both elo's and let the game pick two fitting stealo rules.
        Rules were taken from (or inspired by) <a className="text-blue-900" href="https://www.elostealo.com">elostealo.com</a></div>
    </div>)
}