Imports System.Data
Imports System.Runtime.CompilerServices

Public Module XMLComments

	Public Enum TableComments
		Clothes = 0
		AmmoTypes
	End Enum

	Public Comments() As String = {"
	Possible vests:
		YELLOWVEST
		BLACKSHIRT
		REDVEST
		GREENVEST
		JEANVEST
		BLUEVEST
		greyVEST  :: note capitalization here
		GYELLOWSHIRT
		WHITEVEST
		PURPLESHIRT
		BLUEVEST
		BROWNVEST
	
	Possible pants:
		GREENPANTS
		BLACKPANTS
		BEIGEPANTS
		TANPANTS
		JEANPANTS
		BLUEPANTS

",
"
	This xml defines the properties of different ammo types. 
	
	<uiIndex>			ammotype number. Referred to with <ubAmmoType> in Magazines.xml
	
	<red>				RGB colour values (0-255) when displaying ammo count
	<green>
	<blue>
	
	<structureImpactReductionMultiplier> / <structureImpactReductionDivisor>
						modifies how much bullet damage is reduced when hitting a structure.
						
	<armourImpactReductionMultiplier> / <armourImpactReductionDivisor>
						modifies protection value of body armour we hit with the bullet
						
	<beforeArmourDamageMultiplier> / <beforeArmourDamageDivisor>
						modifies damage before armour is hit. Is only used if <highExplosive> is > 0
	
	<afterArmourDamageMultiplier> / <afterArmourDamageDivisor>
						modifies damage after armour is hit. Not used against armed vehicles.
						
	<zeroMinimumDamage>	0/1 setting. If set to 1, bullet damage can be 0.
	<usPiercePersonChanceModifier>
						if > 0, extra chance for bullet to continue flying if gun isn't a rocket launcher or cannon (and bullet is still powerful enough)
	<standardIssue>		0/1 setting. If set to 1, magazines  of this ammo are considered when determining ammo for the AI inventory.
	<numberOfBullets>	number of projectiles fired per shot. If > 1, this is considered buckshot
	
	<multipleBulletDamageMultiplier> / <multipleBulletDamageDivisor>
						modifies damage immediately when fired if > 1 projectile is fired per bullet
						
	<highExplosive>		number of explosive item used when exploding
	<explosionSize>		explosion size: 0=none, 1=small, 2=medium, 3=large
	<dart>				0/1 setting. If set to 1, damage dealt > 0, target is not an armed vehicle and is hit very accurate, target might be put to sleep.
	<knife>				0/1 setting. If set to 1, target is seen by shooter, bullet is not a fragment, and this is a stealth attack, chance to instakill the target
	<monsterSpit>		bullet gets increased damage according to creature gas items, can blind targets, damage head gear and spawn creature gas
	<acidic>			0/1 setting. If set to 1, armour is destroyed 4 times as fast
	<ignoreArmour>		0/1 setting. If set to 1, torso and leg armour is ignored
	<lockBustingPower>	extra damage done to locks
	<tracerEffect>		0/1 setting. If set to 1, bullets can be tracers. Flash suppressors don't work, not selected on AI single shot guns.
	<spreadPattern />	spread pattern used from SpreadPatterns.xml
	<temperatureModificator>	additive modifier for temperature generation. 1.0 means +100%
	<dirtModificator>			additive modifier for dirt generation. 1.0 means +100%
	<ammoflag>			flagmask for various ammo properties:
							AMMO_CRYO				1			this ammo shock-freezes target (scifi)
							AMMO_BLIND				2			this ammo will blind if it hits the head
							AMMO_ANTIMATERIEL		4			this ammo is anti-materiel, bullets can destroy structures
							AMMO_TRAIL_WHITESMOKE	8			this ammo leaves a trail of white smoke
							AMMO_TRAIL_FIRE			16			this ammo leaves a trail of fire

	<dDamageModifierLife>					multiplier for damage to health
	<dDamageModifierBreath>					multiplier for damage to breath
	<dDamageModifierTank>					additional multiplier for damage to health if target is a tank
	<dDamageModifierArmouredVehicle>		additional multiplier for damage to health if target is not a tank but armed vehicle or robot
	<dDamageModifierCivilianVehicle>		additional multiplier for damage to health if target is unarmed vehicle
	<dDamageModifierZombie>					additional multiplier for damage to health if target is zombie
	<shotAnimation> 	allows defining custom shot animation, for example add <shotAnimation>TILECACHE\MINIBOOM.sti</shotAnimation>
"
	}

End Module
